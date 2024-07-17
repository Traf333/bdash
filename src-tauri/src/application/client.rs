use log::info;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use serde_json::json;
use std::error::Error;

use crate::domain::account::{Account, Data};

fn head(access_token: &Option<String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko)"));
    headers.insert(
        "Origin",
        HeaderValue::from_static("https://telegram.blum.codes"),
    );
    if let Some(ref token) = access_token {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
    headers
}

pub async fn init(query: &serde_json::Value) -> Result<(String, String), Box<dyn Error>> {
    let client = Client::new();
    dbg!(&json!({ "query": query }));
    let resp = client
        .post("https://gateway.blum.codes/v1/auth/provider/PROVIDER_TELEGRAM_MINI_APP")
        .json(&json!({ "query": query }))
        .headers(head(&None))
        .send()
        .await?;
    let resp_json: serde_json::Value = resp.json().await?;
    dbg!(&resp_json);
    let token_data = &resp_json["token"];
    Ok((
        token_data["refresh"].as_str().unwrap().to_string(),
        token_data["access"].as_str().unwrap().to_string(),
    ))
}
pub async fn daily_claim(account: &Account) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    client
        .post("https://game-domain.blum.codes/api/v1/daily-reward?offset=-180")
        .headers(head(&account.access_token))
        .send()
        .await?;
    Ok(())
}

pub async fn eight_hours_claim(account: &Account) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    client
        .post("https://game-domain.blum.codes/api/v1/farming/claim")
        .headers(head(&account.access_token))
        .send()
        .await?;
    client
        .post("https://game-domain.blum.codes/api/v1/farming/start")
        .headers(head(&account.access_token))
        .send()
        .await?;
    Ok(())
}

pub async fn balance(account: &mut Account) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let uri = "https://game-domain.blum.codes/api/v1/user/balance";
    let resp = client
        .get(uri)
        .headers(head(&account.access_token))
        .send()
        .await?;
    let resp_json: serde_json::Value = resp.json().await?;
    info!(
        "{}, play passes {}, balance: {}",
        account.name, resp_json["playPasses"], resp_json["availableBalance"]
    );
    let available_balance: f64 = match resp_json["availableBalance"].as_str() {
        Some(balance) => balance.parse().unwrap_or(0.0),
        None => 0.0,
    };
    account.data = Some(Data {
        play_passes: resp_json["playPasses"].as_u64().unwrap_or(0),
        total_balance: available_balance,
    });
    Ok(())
}

pub async fn refresh_token(token: &String) -> Result<(String, String), Box<dyn Error>> {
    let client = Client::new();

    let resp = client
        .post("https://gateway.blum.codes/v1/auth/refresh")
        .json(&json!({ "refresh": token }))
        .headers(head(&None))
        .send()
        .await?;
    let resp_json: serde_json::Value = resp.json().await?;
    let access_token = &resp_json["access"];
    let refresh_token = &resp_json["refresh"];

    dbg!(access_token);
    Ok((
        refresh_token.as_str().unwrap().to_string(),
        access_token.as_str().unwrap().to_string(),
    ))
}
