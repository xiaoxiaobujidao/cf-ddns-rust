use anyhow::Result;
use cf_ddns_rust::{cloudflare::CloudflareClient, config::Config, get_real_ip};
use std::time::Duration;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::new()?;
    log::info!("Config loaded: {:?}", config);

    // 验证必要的配置
    if config.token.is_empty() || config.domain.is_empty() || config.root_domain.is_empty() {
        log::error!("Missing required configuration: token, domain and root_domain must be set");
        return Ok(());
    }

    let cf_client = CloudflareClient::new(config.token.clone());

    // 获取 zone ID
    let zone_id = match cf_client.get_zone_id(&config.root_domain).await {
        Ok(id) => {
            log::info!("Found zone ID: {} for root domain: {}", id, config.root_domain);
            id
        }
        Err(e) => {
            log::error!("Failed to get zone ID for root domain {}: {}", config.root_domain, e);
            return Ok(());
        }
    };

    log::info!("Starting DDNS client");

    loop {
        log::info!("Checking for IP changes");

        if config.ipv4 {
            match get_real_ip::get_ipv4().await {
                Ok(ip) => {
                    log::info!("Current IPv4: {}", ip);
                    if let Err(e) = cf_client
                        .update_or_create_record(&zone_id, &config.domain, "A", &ip)
                        .await
                    {
                        log::error!("Failed to update IPv4 record: {}", e);
                    }
                }
                Err(e) => log::error!("Failed to get IPv4: {}", e),
            }
        }

        if config.ipv6 {
            match get_real_ip::get_ipv6().await {
                Ok(ip) => {
                    log::info!("Current IPv6: {}", ip);
                    if let Err(e) = cf_client
                        .update_or_create_record(&zone_id, &config.domain, "AAAA", &ip)
                        .await
                    {
                        log::error!("Failed to update IPv6 record: {}", e);
                    }
                }
                Err(e) => log::error!("Failed to get IPv6: {}", e),
            }
        }

        // 生成1-300秒的随机等待时间（1秒到5分钟）
        let mut rng = rand::thread_rng();
        let wait_seconds = rng.gen_range(1..=300);
        log::info!("Waiting {} seconds before next check", wait_seconds);
        tokio::time::sleep(Duration::from_secs(wait_seconds)).await;
    }
}
