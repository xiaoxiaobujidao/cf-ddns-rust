use anyhow::Result;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Clone, Copy)]
enum IpFamily {
    V4,
    V6,
}

#[derive(Deserialize, Debug)]
struct IpResponse {
    ip: String,
}

fn client_for(family: IpFamily) -> Result<reqwest::Client> {
    // 绑定到对应协议栈的未指定地址，强制只走 IPv4 或 IPv6
    let local_address = match family {
        IpFamily::V4 => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        IpFamily::V6 => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
    };

    Ok(reqwest::Client::builder()
        .local_address(local_address)
        .build()?)
}

fn ip_matches_family(ip: IpAddr, family: IpFamily) -> bool {
    match family {
        IpFamily::V4 => ip.is_ipv4(),
        IpFamily::V6 => ip.is_ipv6(),
    }
}

fn parse_ip_from_text(text: &str, family: IpFamily) -> Result<String> {
    // 尝试解析为 JSON
    if let Ok(ip_response) = serde_json::from_str::<IpResponse>(text) {
        let ip = ip_response.ip.trim().parse::<IpAddr>()?;
        if ip_matches_family(ip, family) {
            return Ok(ip.to_string());
        }
        return Err(anyhow::anyhow!(
            "IP family mismatch, expected {}, got {}",
            family_name(family),
            ip
        ));
    }

    // 如果不是 JSON，可能是纯文本 IP
    let ip = text.trim().parse::<IpAddr>()?;
    if ip_matches_family(ip, family) {
        return Ok(ip.to_string());
    }

    Err(anyhow::anyhow!(
        "IP family mismatch, expected {}, got {}",
        family_name(family),
        ip
    ))
}

fn family_name(family: IpFamily) -> &'static str {
    match family {
        IpFamily::V4 => "IPv4",
        IpFamily::V6 => "IPv6",
    }
}

async fn get_ip_from_service(
    client: &reqwest::Client,
    url: &str,
    family: IpFamily,
) -> Result<String> {
    let resp = client.get(url).send().await?;
    let text = resp.text().await?;
    parse_ip_from_text(&text, family)
}

pub async fn get_ipv4() -> Result<String> {
    let ipv4_services = vec![
        "https://api.ipify.org?format=json",
        "https://ipinfo.io/ip",
        "https://icanhazip.com",
        "https://checkip.amazonaws.com",
        "https://ip.bujidao.org/json",
    ];

    let client = client_for(IpFamily::V4)?;

    for service in ipv4_services {
        log::debug!("Trying IPv4 service: {}", service);
        match get_ip_from_service(&client, service, IpFamily::V4).await {
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
        "https://ip.bujidao.org/json",
    ];

    let client = client_for(IpFamily::V6)?;

    for service in ipv6_services {
        log::debug!("Trying IPv6 service: {}", service);
        match get_ip_from_service(&client, service, IpFamily::V6).await {
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
