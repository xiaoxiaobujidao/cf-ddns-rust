use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub domain: String,
    pub root_domain: String,
    pub ipv4: bool,
    pub ipv6: bool,
    pub check_cn_connectivity: bool,
    /// 两次检查之间的最大随机等待秒数（最小为 1 秒）
    pub max_wait_seconds: u64,
    pub token: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let builder = config::Config::builder()
            .set_default("domain", "")?
            .set_default("root_domain", "")?
            .set_default("ipv4", true)?
            .set_default("ipv6", true)?
            .set_default("check_cn_connectivity", false)?
            .set_default("max_wait_seconds", 300)?
            .set_default("token", "")?
            .add_source(config::File::with_name("config").required(false))
            // .add_source(config::Environment::with_prefix("").separator("_"));
            .add_source(config::Environment::default());

        let config = builder.build()?.try_deserialize()?;
        Ok(config)
    }
}
