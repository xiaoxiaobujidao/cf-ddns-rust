use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IpResponse {
    ip: String,
}

#[derive(Deserialize, Debug)]
struct IpifyResponse {
    ip: String,
}

#[derive(Deserialize, Debug)]
struct IpinfoResponse {
    ip: String,
}

async fn get_ip_from_service(url: &str) -> Result<String> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    
    // 尝试解析为 JSON
    if let Ok(ip_response) = serde_json::from_str::<IpResponse>(&text) {
        return Ok(ip_response.ip);
    }
    
    // 如果不是 JSON，可能是纯文本 IP
    let ip = text.trim();
    if ip.parse::<std::net::IpAddr>().is_ok() {
        return Ok(ip.to_string());
    }
    
    Err(anyhow::anyhow!("Failed to parse IP from response: {}", text))
}

pub async fn get_ipv4() -> Result<String> {
    let ipv4_services = vec![
        "https://api.ipify.org?format=json",
        "https://ipinfo.io/ip",
        "https://icanhazip.com",
        "https://checkip.amazonaws.com",
    ];
    
    for service in ipv4_services {
        log::debug!("Trying IPv4 service: {}", service);
        match get_ip_from_service(service).await {
            Ok(ip) => {
                log::info!("Successfully got IPv4 from {}: {}", service, ip);
                return Ok(ip);
            }
            Err(e) => {
                log::warn!("Failed to get IPv4 from {}: {}", service, e);
                continue;
            }
        }
    }
    
    Err(anyhow::anyhow!("All IPv4 services failed"))
}

pub async fn get_ipv6() -> Result<String> {
    let ipv6_services = vec![
        "https://api64.ipify.org?format=json",
        "https://ipv6.icanhazip.com",
        "https://v6.ident.me",
    ];
    
    for service in ipv6_services {
        log::debug!("Trying IPv6 service: {}", service);
        match get_ip_from_service(service).await {
            Ok(ip) => {
                log::info!("Successfully got IPv6 from {}: {}", service, ip);
                return Ok(ip);
            }
            Err(e) => {
                log::warn!("Failed to get IPv6 from {}: {}", service, e);
                continue;
            }
        }
    }
    
    Err(anyhow::anyhow!("All IPv6 services failed"))
}