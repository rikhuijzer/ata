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

    let mut response = client.request(req).await?;

    print_and_flush("\n");
    print_response();

    let mut data_buffer = vec![];
    let mut print_buffer: Vec<String> = vec![];
    while let Some(chunk) = response.body_mut().data().await {

        let chunk = chunk?;
        data_buffer.extend_from_slice(&chunk);

        let events = std::str::from_utf8(&data_buffer)?.split("\n\n");
        for event in events {
            if event.starts_with("data:") {
                let data = &event[6..];
                if data == "[DONE]" {
                    return Ok(finish_prompt(is_running));
                };
                let v: Value = serde_json::from_str(&data)?;
                let text: &str = v["choices"][0]["text"].as_str().unwrap();
                let processed = post_process(&mut print_buffer, text);
                print_and_flush(&processed);
            }
            if abort.load(Ordering::SeqCst) {
                abort.store(false, Ordering::SeqCst);
                return Ok(finish_prompt(is_running));
            }
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
}
