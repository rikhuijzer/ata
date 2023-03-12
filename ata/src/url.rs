use crate::Config;
use hyper::Body;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use crate::prompt::request;
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
use hyper::Uri;
use std::str::FromStr;
use url::Url;

async fn web_page(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    // add http if url doesn't have a scheme
    let url = match url.contains("://") {
        true => url.to_string(),
        false => format!("http://{url}")
    };
    println!("{url}");
    let uri = Uri::from_str(&url)?;

    let mut response = client.get(uri).await?;

    // Read the response body
    let mut body = String::new();
    while let Some(chunk) = response.body_mut().data().await {
        let chunk = chunk?;
        body.push_str(std::str::from_utf8(&chunk)?);
    }

    Ok(body)
}

#[tokio::main]
pub async fn request_from_url(config: &Config, url: String) -> String {
    let abort = Arc::new(AtomicBool::new(false));
    let is_running = Arc::new(AtomicBool::new(false));
    let count = 0;
    // request a http with hyper
    let html = web_page(&url).await.unwrap();

    println!("html: {html}");

    let prompt = html;

    let result = request(abort.clone(), is_running.clone(), config, prompt.clone(), count).unwrap();
    "".to_string()
}
