use std::env;
use dotenvy::dotenv;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {

    dotenv().ok();

    // เปลี่ยนค่าเหล่านี้
    let api_token: String = env::var("API_TOKEN").expect("API_TOKEN not set");
    let zone_id: String = env::var("ZONE_ID").expect("ZONE_ID not set");
    let record_id: String = env::var("RECORD_ID").expect("RECORD_ID not set");
    let domain: String = env::var("DOMAIN").expect("DOMAIN not set");
    
    let ip: String = reqwest::get("https://api.ipify.org").await?.text().await?;

    // HTTP PUT Request ไป Cloudflare (ง่ายที่สุด)
    let client = Client::new();
    let res = client
        .put(format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        ))
        .bearer_auth(api_token)
        .json(&json!({
            "type": "A",
            "name": domain,
            "content": ip,
            "ttl": 120,
            "proxied": false
        }))
        .send()
        .await?;

    println!("Cloudflare response: {:?}", res.text().await?);
    Ok(())
}
