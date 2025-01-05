use reqwest::blocking::Client as SyncClient;
use reqwest::Client as AsyncClient;
use mockito::{mock, Matcher};
use std::error::Error;
use serde_json::json;
use tokio::runtime::Runtime;

pub struct PushinatorClient {
    api_token: String,
}

impl PushinatorClient {
    pub fn new(api_token: String) -> Self {
        PushinatorClient {
            api_token: api_token.to_string(),
        }
    }

    pub fn send_notification_sync(&self, channel_id: String, notification: &str) -> Result<(), Box<dyn Error>> {
        let api_url = "https://api.pushinator.com/api/v1/send_notification";

    let client = SyncClient::new();

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", self.api_token))
        .header("Content-Type", "application/json")
        .header("User-Agent", "pushinator-rust/1.0")
        .json(&json!({
            "channel": channel_id,
            "notification": notification
        }))
        .send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_body = response.text().unwrap();
        Err(From::from(format!(
            "Failed to send notification. Status: {}, Body: {}",
            status,
            error_body
        )))
    }
    }

    pub async fn send_notification(&self, channel_id: String, notification: &str) -> Result<(), Box<dyn Error>> {
        let api_url = "https://api.pushinator.com/api/v1/send_notification";

    let client = AsyncClient::new();

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", self.api_token))
        .header("Content-Type", "application/json")
        .header("User-Agent", "pushinator-rust/1.0")
        .json(&json!({
            "channel": channel_id,
            "notification": notification
        }))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_body = response.text().await?;
        Err(From::from(format!(
            "Failed to send notification. Status: {}, Body: {}",
            status,
            error_body
        )))
    }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn test_send_notification_sync_success() {
        let _mock = mock("POST", "/api/v1/send_notification")
            .match_header("Authorization", Matcher::Regex("Bearer .+".to_string()))
            .match_header("Content-Type", "application/json")
            .match_header("User-Agent", "pushinator-rust/1.0")
            .match_body(Matcher::Json(json!({
                "channel": "test-channel-id",
                "notification": "Test notification"
            })))
            .with_status(200)
            .create();

        let client = PushinatorClient::new("test-api-token".to_string());
        let result = client.send_notification_sync("test-channel-id".to_string(), "Test notification");

        assert!(result.is_ok());
    }

    #[test]
    fn test_send_notification_sync_failure() {
        let _mock = mock("POST", "/api/v1/send_notification")
            .match_header("Authorization", Matcher::Regex("Bearer .+".to_string()))
            .match_header("Content-Type", "application/json")
            .match_header("User-Agent", "pushinator-rust/1.0")
            .match_body(Matcher::Json(json!({
                "channel": "test-channel-id",
                "notification": "Test notification"
            })))
            .with_status(500)
            .with_body("{\"error\":\"Internal Server Error\"}")
            .create();

        let client = PushinatorClient::new("test-api-token".to_string());
        let result = client.send_notification_sync("test-channel-id".to_string(), "Test notification");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_notification_async_success() {
        let _mock = mock("POST", "/api/v1/send_notification")
            .match_header("Authorization", Matcher::Regex("Bearer .+".to_string()))
            .match_header("Content-Type", "application/json")
            .match_header("User-Agent", "pushinator-rust/1.0")
            .match_body(Matcher::Json(json!({
                "channel": "test-channel-id",
                "notification": "Test notification"
            })))
            .with_status(200)
            .create();

        let client = PushinatorClient::new("test-api-token".to_string());
        let result = client.send_notification("test-channel-id".to_string(), "Test notification").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_notification_async_failure() {
        let _mock = mock("POST", "/api/v1/send_notification")
            .match_header("Authorization", Matcher::Regex("Bearer .+".to_string()))
            .match_header("Content-Type", "application/json")
            .match_header("User-Agent", "pushinator-rust/1.0")
            .match_body(Matcher::Json(json!({
                "channel": "test-channel-id",
                "notification": "Test notification"
            })))
            .with_status(500)
            .with_body("{\"error\":\"Internal Server Error\"}")
            .create();

        let client = PushinatorClient::new("test-api-token".to_string());
        let result = client.send_notification("test-channel-id".to_string(), "Test notification").await;

        assert!(result.is_err());
    }
}
