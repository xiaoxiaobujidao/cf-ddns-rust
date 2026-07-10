use anyhow::Result;
use std::time::Duration;

const CSDN_URL: &str = "http://csdn.net";

pub async fn check_cn_connectivity() -> Result<bool> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    client.get(CSDN_URL).send().await?;
    Ok(true)
}
