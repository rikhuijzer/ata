use hyper::Body;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use termion::raw::IntoRawMode;
use std::path::Path;
use std::result::Result;
use toml::from_str;
use termion::event::Key;
use termion::event::MouseEvent;
use termion::event::Event;
use termion::input::TermRead;
use termion::input::MouseTerminal;

#[derive(Clone, Deserialize, Debug)]
struct Config {
    api_key: String,
    model: String,
    max_tokens: i64,
    temperature: i64
}

type TokioResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

#[tokio::main]
async fn prompt_model(config: Config, prompt: String) -> TokioResult<String> {
    let api_key: String = config.clone().api_key;
    let model: String = config.clone().model;
    let max_tokens: i64 = config.clone().max_tokens;
    let temperature: i64 = config.temperature;

    let mut sanitized_input: String = prompt.clone();
    sanitized_input.pop();
    sanitized_input = sanitized_input.replace("\"", "\\\"");
    let bearer = format!("Bearer {}", api_key);
    // Passing newlines behind the prompt to get a more chat-like experience.
    let body = json!({
        "model": model,
        "prompt": format!("{}\\n\\n", sanitized_input),
        "max_tokens": max_tokens,
        "temperature": temperature
    }).to_string();
    // println!("{}", body);

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer)
        .body(Body::from(body))?;

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let resp = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

    // println!("{}", String::from_utf8(body_bytes.clone().to_vec()).unwrap());

    let v: Value = serde_json::from_slice(&body_bytes)?;
    if v.get("error").is_some() {
        let text: String = v["error"]["message"].to_string();
        Ok(text)
    } else {
        let text: String = v["choices"][0]["text"].to_string();
        Ok(text)
    }
}

fn remove_quotation_marks(mut text: String) -> String {
    text.pop();
    text.remove(0);
    text
}

fn remove_leading_newlines(text: String) -> String {
    let re = regex::Regex::new(r"^[\n]*").unwrap();
    re.replace_all(&text, "").into_owned()
}

fn sanitize_response(response: String) -> String {
    let mut text = response;
    text = text.replace("\\n", "\n");
    text = remove_quotation_marks(text);
    remove_leading_newlines(text)
}

fn main() -> TokioResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <Path to termgpt.toml>", args[0]);
        return Ok(());
    }

    let file = args[1].to_string();
    // println!("Argument: {}", file);
    if !Path::new(&file).exists() {
        panic!("Couldn't find file: {}", file);
    }

    let mut contents = String::new();
    File::open(file).unwrap().read_to_string(&mut contents).unwrap();

    let config: Config = from_str(&contents).unwrap();
    // println!("{:?}", config);

    let model: String = config.clone().model;

    let stdout = std::io::stdout();
    let mut stdout = MouseTerminal::from(stdout.into_raw_mode().unwrap());

    let stdin = std::io::stdin();

        // print!("{}> ", model);
        write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    Ok(())
}
