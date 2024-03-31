use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
//TODO: give me a better name
pub enum CustomError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid response status code: {0}")]
    InvalidStatusCode(reqwest::StatusCode),
    #[error("Failed to parse response body")]
    ParseError,
}
pub type Result<T> = std::result::Result<T, CustomError>;

use reqwest::Client;

//deprecated
pub async fn query(url: &str, topic: &str, query: &str) -> Result<String> {
    let client = Client::new();
    //body is a json:
    //'{"topic":"faust", "query":"how clickhouse is used ?"}'
    let body = json!({
        "topic": topic,
        "query": query,
    });

    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await?;

    //print!("{:?}", response);
    if !response.status().is_success() {
        return Err(CustomError::InvalidStatusCode(response.status()));
    }
    let body = response.text().await?;
    Ok(body)
}
