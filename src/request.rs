// use std::collections::HashMap;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::env;

pub async fn request<T: DeserializeOwned + std::fmt::Debug>(
    url: &str,
    query: Option<&[(&str, &str)]>,
) -> Result<T, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let bearer_token = env::var("TWITTER_BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN not found");

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", bearer_token))?,
    );

    let client = Client::new();
    let resp;

    if query.is_some() {
        resp = client
            .get(url)
            .headers(headers)
            .query(&query)
            .send()
            .await?
            .json::<T>()
            .await?;
    } else {
        resp = client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?;
    }

    Ok(resp)
}
