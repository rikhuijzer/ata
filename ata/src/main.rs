use clap::Parser;
use hyper::Body;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use hyper::body::HttpBody;
use hyper_rustls::HttpsConnectorBuilder;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::result::Result;
use toml::from_str;

mod help;

#[derive(Clone, Deserialize, Debug)]
struct Config {
    api_key: String,
    model: String,
    max_tokens: i64,
    temperature: i64
}

type TokioResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

fn sanitize_input(input: String) -> String {
    let mut out = input.clone();
    out.pop();
    out = out.replace("\"", "\\\"");
    out
}

#[tokio::main]
async fn prompt_model(config: Config, prompt: String) -> TokioResult<()> {
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

    println!("");

    let mut buffer = vec![];
    while let Some(chunk) = response.body_mut().data().await {
        let chunk = chunk?;
        buffer.extend_from_slice(&chunk);

        let events = std::str::from_utf8(&buffer)?.split("\n\n");
        for event in events {
            if event.starts_with("data:") {
                let data = &event[6..];
                if data == "[DONE]" {
                    println!("\n");
                    return Ok(());
                };
                let v: Value = serde_json::from_str(&data)?;
                let text = v["choices"][0]["text"].as_str().unwrap();
                print!("{}", text);
                std::io::stdout().flush().unwrap();
            }
        }
        buffer.clear();
    };
    println!("\n");
    Ok(())
}

fn missing_toml(args: Vec<String>) {
    eprintln!(
        "Use `{} --config=<Path to ata.toml>` or have `ata.toml` in the current dir.",
        args[0]
    );
    std::process::exit(1);
}

/// Ask the Terminal Anything (ATA): OpenAI GPT in the terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Flags {
    /// Path to the configuration TOML file.
    #[arg(short = 'c', long = "config", default_value = "ata.toml")]
    config: String,

    /// Print the keyboard shortcuts.
    #[arg(long)]
    print_shortcuts: bool,
}

fn main() -> TokioResult<()> {
    let args: Vec<String> = env::args().collect();
    let flags: Flags = Flags::parse();
    if flags.print_shortcuts {
        help::commands();
        return Ok(());
    }
    let filename = flags.config;
    if !Path::new(&filename).exists() {
        missing_toml(args);
    }
    let mut contents = String::new();
    File::open(filename).unwrap().read_to_string(&mut contents).unwrap();

    let config: Config = from_str(&contents).unwrap();

    println!("Ask the Terminal Anything");

    let mut rl = Editor::<()>::new()?;

    loop {
        let readline = rl.readline(&format!("{}> ", config.clone().model));
        match readline {
            Ok(line) => {
                if line == "" {
                    continue
                }
                rl.add_history_entry(line.as_str());
                prompt_model(config.clone(), line)?;
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                eprintln!("{:?}", err);
                break
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_newlines() {
        assert_eq!(sanitize_input("foo\"bar".to_string()), "foo\\\"ba".to_string());
    }
}
