use reqwest;
use serde;
use std::fs;
use std::io;
use std::path;

#[derive(serde::Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}
pub async fn check_for_updates() -> Result<Option<String>, reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response = client
        .get("https://github.com/whittte169/Aurum")
        .header(reqwest::header::USER_AGENT, "whittte169")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await?;
    if response.status().is_success() {
        let json: Release = response.json().await?;
        let exe_asset = json
            .assets
            .iter()
            .find(|a| a.name.ends_with(".exe"));
        match exe_asset {
            Some(asset) => Ok(Some(asset.browser_download_url.clone())),
            None => {
                println!(".exe file isn't found in the latest release!");
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}