use anyhow::Result;

use serde::Deserialize;

#[derive(Deserialize)]
struct ClientConfig {
    #[serde(rename = "keystone.self_update.manifest_url")]
    manifest_url: String,
}

pub async fn get_manifest_url() -> Result<String> {
    let host = r"clientconfig.rpg.riotgames.com/api/v1/config/public";
    let params = "version=99.0.0.9999999&patchline=KeystoneFoundationLiveWin&app=Riot%20Client&namespace=keystone.self_update";
    let url = format!("https://{}?{}", host, params);
    let res = reqwest::get(url).await?;
    let body = res.bytes().await?;
    let data: ClientConfig = serde_json::from_slice(&body)?;
    let manifest_url = data.manifest_url;
    Ok(manifest_url)
}
