use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CloudflareResponse<T> {
    success: bool,
    errors: Vec<CloudflareError>,
    messages: Vec<String>,
    result: Option<T>,
}

#[derive(Debug, Deserialize)]
struct CloudflareError {
    code: u32,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DnsRecord {
    id: Option<String>,
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: u32,
}

#[derive(Debug, Deserialize)]
struct Zone {
    id: String,
    name: String,
}

pub struct CloudflareClient {
    client: reqwest::Client,
    token: String,
}

impl CloudflareClient {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
        }
    }

    async fn make_request<T>(&self, method: reqwest::Method, url: &str, body: Option<&str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut request = self
            .client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", &self.token))
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.body(body.to_string());
        }

        let response = request.send().await?;
        let text = response.text().await?;
        
        let cf_response: CloudflareResponse<T> = serde_json::from_str(&text)
            .map_err(|e| anyhow!("Failed to parse response: {}, body: {}", e, text))?;

        if !cf_response.success {
            let error_msg = cf_response
                .errors
                .into_iter()
                .map(|e| format!("Code {}: {}", e.code, e.message))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(anyhow!("Cloudflare API error: {}", error_msg));
        }

        cf_response.result.ok_or_else(|| anyhow!("No result in response"))
    }

    pub async fn get_zone_id(&self, domain: &str) -> Result<String> {
        let url = format!("https://api.cloudflare.com/client/v4/zones?name={}", domain);
        let zones: Vec<Zone> = self.make_request(reqwest::Method::GET, &url, None).await?;
        
        zones
            .into_iter()
            .find(|zone| zone.name == domain)
            .map(|zone| zone.id)
            .ok_or_else(|| anyhow!("Zone not found for domain: {}", domain))
    }

    pub async fn get_dns_record(&self, zone_id: &str, name: &str, record_type: &str) -> Result<Option<DnsRecord>> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}&type={}",
            zone_id, name, record_type
        );
        
        let records: Vec<DnsRecord> = self.make_request(reqwest::Method::GET, &url, None).await?;
        Ok(records.into_iter().next())
    }

    pub async fn create_dns_record(&self, zone_id: &str, name: &str, record_type: &str, content: &str) -> Result<DnsRecord> {
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", zone_id);
        
        let record = DnsRecord {
            id: None,
            record_type: record_type.to_string(),
            name: name.to_string(),
            content: content.to_string(),
            ttl: 300,
        };
        
        let body = serde_json::to_string(&record)?;
        self.make_request(reqwest::Method::POST, &url, Some(&body)).await
    }

    pub async fn update_dns_record(&self, zone_id: &str, record_id: &str, name: &str, record_type: &str, content: &str) -> Result<DnsRecord> {
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, record_id);
        
        let record = DnsRecord {
            id: Some(record_id.to_string()),
            record_type: record_type.to_string(),
            name: name.to_string(),
            content: content.to_string(),
            ttl: 300,
        };
        
        let body = serde_json::to_string(&record)?;
        self.make_request(reqwest::Method::PUT, &url, Some(&body)).await
    }

    pub async fn update_or_create_record(&self, zone_id: &str, name: &str, record_type: &str, content: &str) -> Result<()> {
        match self.get_dns_record(zone_id, name, record_type).await? {
            Some(existing_record) => {
                if existing_record.content != content {
                    log::info!("Updating {} record for {} from {} to {}", record_type, name, existing_record.content, content);
                    self.update_dns_record(zone_id, &existing_record.id.unwrap(), name, record_type, content).await?;
                    log::info!("Successfully updated {} record for {}", record_type, name);
                } else {
                    log::info!("{} record for {} is already up to date", record_type, name);
                }
            }
            None => {
                log::info!("Creating new {} record for {} with content {}", record_type, name, content);
                self.create_dns_record(zone_id, name, record_type, content).await?;
                log::info!("Successfully created {} record for {}", record_type, name);
            }
        }
        Ok(())
    }
}