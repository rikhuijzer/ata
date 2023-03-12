use crate::Config;
use hyper::Body;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use hyper::body::HttpBody;
use hyper_rustls::HttpsConnectorBuilder;
use serde_json::Value;
use serde_json::json;
use std::error::Error;
use std::io::Write;
use std::result::Result;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub type TokioResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

fn sanitize_input(input: String) -> String {
    let mut out = input;
    out.pop();
    out.replace('"', "\\\"")
}

fn print_and_flush(text: &str) {
    print!("{text}");
    std::io::stdout().flush().unwrap();
}

pub fn print_bold(msg: &str) {
    println!("\x1b[1m{msg}\x1b[0m");
}

pub fn print_prompt() {
    print_bold("Prompt: ");
}

fn print_response() {
    print_bold("Response: ");
}

pub fn finish_prompt(is_running: Arc<AtomicBool>) {
    is_running.store(false, Ordering::SeqCst);
    print_and_flush("\n\n");
    print_prompt();
}

pub fn print_error(is_running: Arc<AtomicBool>, msg: &str) {
    eprintln!("\x1b[1mError: \x1b[0m \n{msg}");
    finish_prompt(is_running)
}

fn store_and_do_nothing(print_buffer: &mut Vec<String>, text: &str) -> String {
    print_buffer.push(text.to_string());
    "".to_string()
}

fn join_and_clear(print_buffer: &mut Vec<String>, text: &str) -> String {
    let from_buffer = print_buffer.join("");
    print_buffer.clear();
    let joined = format!("{from_buffer}{text}");
    joined.replace("\\n", "\n")
}

// Fixes cases where the model returns ["\", "n"] instead of ["\n"],
// which is interpreted as a newline in the OpenAI playground.
fn fix_newlines(print_buffer: &mut Vec<String>, text: &str) -> String {
    let single_backslash = r#"\"#;
    if text.ends_with(single_backslash) {
        return store_and_do_nothing(print_buffer, text);
    }
    if !print_buffer.is_empty() {
        return join_and_clear(print_buffer, text);
    }
    text.to_string()
}

fn post_process(print_buffer: &mut Vec<String>, text: &str) -> String {
    fix_newlines(print_buffer, text)
}

fn value2unquoted_text(value: &serde_json::Value) -> String {
    value.as_str().unwrap().to_string()
}

fn should_retry(line: &str, count: i64) -> bool {
    let v: Value = match serde_json::from_str(line) {
        Ok(line) => line,
        Err(_) => return false,
    };
    if v.get("error").is_some() {
        let error_type = value2unquoted_text(&v["error"]["type"]);
        let max_tries = 3;
        if count < max_tries && error_type == "server_error" {
            println!(
                "\
                Server responded with a `server_error`. \
                Trying again... ({count}/{max_tries})\
                "
            );
            return true;
        }
    }
    false
}

#[tokio::main]
pub async fn request(
    abort: Arc<AtomicBool>,
    is_running: Arc<AtomicBool>,
    config: &Config,
    prompt: String,
    count: i64,
) -> TokioResult<bool> {
    is_running.store(true, Ordering::SeqCst);

    let api_key: String = config.clone().api_key;
    let model: String = config.clone().model;
    let max_tokens: i64 = config.clone().max_tokens;
    let temperature: f64 = config.temperature;

    let sanitized_input = sanitize_input(prompt.clone());
    let bearer = format!("Bearer {api_key}");
    // Passing newlines behind the prompt to get a more chat-like experience.
    let body = json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": format!("{sanitized_input}\\n\\n")
            }
        ],
        "max_tokens": max_tokens,
        "temperature": temperature,
        "stream": true
    })
    .to_string();

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer)
        .body(Body::from(body))?;

    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut response = match client.request(req).await {
        Ok(response) => response,
        Err(e) => {
            print_and_flush("\n");
            print_error(is_running, &e.to_string());
            return Ok(false);
        }
    };

    // Do not move this in front of the request for UX reasons.
    print_and_flush("\n");

    let mut had_first_success = false;
    let mut data_buffer = vec![];
    let mut print_buffer: Vec<String> = vec![];
    while let Some(chunk) = response.body_mut().data().await {
        let chunk = chunk?;
        data_buffer.extend_from_slice(&chunk);

        let events = std::str::from_utf8(&data_buffer)?.split("\n\n");
        for line in events {
            if line.starts_with("data:") {
                let data: &str = &line[6..];
                if data == "[DONE]" {
                    return Ok(false);
                };
                let v: Value = serde_json::from_str(data)?;

                if v.get("choices").is_some() {
                    let delta = v.get("choices").unwrap()[0].get("delta");
                    if delta.is_none() {
                        // Ignoring wrong responses to avoid crashes.
                        continue;
                    }
                    if delta.unwrap().get("content").is_none() {
                        // Probably switching "role" (`"role":"assistant"`).
                        continue;
                    }
                    let text = value2unquoted_text(&delta.unwrap()["content"]);
                    let processed = post_process(&mut print_buffer, &text);
                    if !had_first_success {
                        had_first_success = true;
                        print_response();
                    };
                    print_and_flush(&processed);
                } else if v.get("error").is_some() {
                    let msg = value2unquoted_text(&v["error"]["message"]);
                    print_error(is_running, &msg);
                    return Ok(false);
                } else {
                    print_error(is_running, data);
                    return Ok(false);
                };
            } else if !line.is_empty() {
                if !had_first_success {
                    let retry = should_retry(line, count);
                    if retry {
                        return Ok(true);
                    } else {
                        print_error(is_running, line);
                        return Ok(false);
                    }
                };
                print_error(is_running, line);
                return Ok(false);
            };
            if abort.load(Ordering::SeqCst) {
                abort.store(false, Ordering::SeqCst);
                return Ok(false);
            };
        }
        data_buffer.clear();
    }
    Ok(false)
}

pub fn request_from_url(config: &Config, prompt: String) -> TokioResult<String> {
    let abort = Arc::new(AtomicBool::new(false));
    let is_running = Arc::new(AtomicBool::new(false));
    let count = 0;
    let result = request(abort.clone(), is_running.clone(), config, prompt.clone(), count);
    Ok("foo".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_newlines() {
        assert_eq!(
            sanitize_input("foo\"bar".to_string()),
            "foo\\\"ba".to_string()
        );
    }

    #[test]
    fn value_is_unquoted() {
        use super::*;
        let v: Value = serde_json::from_str(r#"{"a": "1"}"#).unwrap();
        assert_eq!(value2unquoted_text(&v["a"]), "1");
    }
}
