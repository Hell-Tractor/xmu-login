use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;

use super::constants::USER_AGENT;

/// 根据给定的referer创建一个client
///
/// 自动启用cookie
pub async fn create_client(referer: &str) -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert("REFERER", HeaderValue::from_str(referer)?);
    headers.insert("USER_AGENT", HeaderValue::from_static(USER_AGENT));

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .default_headers(headers)
        .build()?;

    Ok(client)
}