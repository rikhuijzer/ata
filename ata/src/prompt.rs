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
    let mut out = input.clone();
    out.pop();
    out = out.replace("\"", "\\\"");
    out
}

fn print_and_flush(text: &str) {
    print!("{text}");
    std::io::stdout().flush().unwrap();
}

pub fn print_prompt() {
    println!("\x1b[1mPrompt: \x1b[0m");
}

fn print_response() {
    println!("\x1b[1mResponse: \x1b[0m");
}

fn finish_prompt(is_running: Arc<AtomicBool>) {
    is_running.store(false, Ordering::SeqCst);
    print_and_flush("\n\n");
    print_prompt();
}

fn print_error(is_running: Arc<AtomicBool>, msg: &str) {
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
    let with_real_newlines = joined.replace("\\n", "\n");
    with_real_newlines
}

// Fixes cases where the model returns ["\", "n"] instead of ["\n"],
// which is interpreted as a newline in the OpenAI playground.
fn fix_newlines(print_buffer: &mut Vec<String>, text: &str) -> String {
    let single_backslash = r#"\"#;
    if text.ends_with(single_backslash) {
        return store_and_do_nothing(print_buffer, text);
    }
    if 0 < print_buffer.len() {
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

#[tokio::main]
pub async fn request(
            abort: Arc<AtomicBool>,
            is_running: Arc<AtomicBool>,
            config: &super::Config,
            prompt: String
        ) -> TokioResult<()> {

    is_running.store(true, Ordering::SeqCst);

    let api_key: String = config.clone().api_key;
    let model: String = config.clone().model;
    let max_tokens: i64 = config.clone().max_tokens;
    let temperature: i64 = config.temperature;

    let sanitized_input = sanitize_input(prompt.clone());
    let bearer = format!("Bearer {}", api_key);
    // Passing newlines behind the prompt to get a more chat-like experience.
    let body = json!({
        "model": model,
        "prompt": format!("{}\\n\\n", sanitized_input),
        "max_tokens": max_tokens,
        "temperature": temperature,
        "stream": true
    }).to_string();

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer)
        .body(Body::from(body))?;

    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let mut response = match client.request(req).await {
        Ok(response) => response,
        Err(e) => {
            let msg = format!("\n{e:?}");
            return Ok(print_error(is_running, &msg));
        }
    };

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
                    return Ok(finish_prompt(is_running));
                };
                let v: Value = serde_json::from_str(&data)?;

                if v.get("choices").is_some() {
                    let text = value2unquoted_text(&v["choices"][0]["text"]);
                    let processed = post_process(&mut print_buffer, &text);
                    if !had_first_success {
                        had_first_success = true;
                        print_response();
                    };
                    print_and_flush(&processed);
                } else if v.get("error").is_some() {
                    let msg = value2unquoted_text(&v["error"]["message"]);
                    return Ok(print_error(is_running, &msg));
                } else {
                    return Ok(print_error(is_running, data));
                };
            } else if line == "" {
            } else {
                return Ok(print_error(is_running, line));
            };
            if abort.load(Ordering::SeqCst) {
                abort.store(false, Ordering::SeqCst);
                return Ok(finish_prompt(is_running));
            };
        }
        data_buffer.clear();
    };
    Ok(finish_prompt(is_running))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_newlines() {
        assert_eq!(sanitize_input("foo\"bar".to_string()), "foo\\\"ba".to_string());
    }

    #[test]
    fn value_is_unquoted() {
        use super::*;
        let v: Value = serde_json::from_str(r#"{"a": "1"}"#).unwrap();
        assert_eq!(value2unquoted_text(&v["a"]), "1");
    }
}
