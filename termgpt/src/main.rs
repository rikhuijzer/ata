use serde::Deserialize;
use hyper::Body;
use std::env;
use std::fs::File;
use serde_json::Value;
use hyper_tls::HttpsConnector;
use hyper::Request;
use hyper::Client;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use toml::from_str;
use hyper::Method;
use std::error::Error;
use std::result::Result;

#[derive(Clone, Deserialize, Debug)]
struct Config {
    api_key: String,
    model: String,
    max_tokens: String
}

type TokioResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

#[tokio::main]
async fn prompt_model(config: Config, prompt: String) -> TokioResult<String> {
    let api_key: String = config.clone().api_key;
    let model: String = config.clone().model;
    let max_tokens: String = config.clone().max_tokens;

    let mut sanitized_input: String = prompt.clone();
    sanitized_input.pop();
    let bearer = format!("Bearer {}", api_key);
    // Passing newlines behind the prompt to get a more chat-like experience.
    let body = format!(
        "{{\"model\": \"{}\", \"prompt\": \"{}\\n\\n\", \"max_tokens\": {}}}",
        model,
        sanitized_input,
        max_tokens
    );
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
    let text: String = v["choices"][0]["text"].to_string();

    Ok(text)
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

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let response = prompt_model(config.clone(), input)?;
        let sanitized = sanitize_response(response);
        print!("{}\n", sanitized);
        stdout().flush().unwrap();
    }
}
