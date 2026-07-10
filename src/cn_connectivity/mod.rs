use anyhow::Result;
use std::time::Duration;

const CSDN_URL: &str = "https://www.csdn.net";

pub async fn check_cn_connectivity() -> Result<bool> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let resp = client.get(CSDN_URL).send().await?;
    let reachable = resp.status().is_success() || resp.status().is_redirection();
    Ok(reachable)
}
