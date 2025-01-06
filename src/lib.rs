use reqwest::blocking::Client as SyncClient;
use reqwest::Client as AsyncClient;

use serde_json::json;
use std::error::Error;
/// A client for interacting with the Pushinator API.

pub struct PushinatorClient {
    /// API token used for authentication.
    api_token: String,
    /// Base URL of the Pushinator API.
    base_url: String,
}

impl PushinatorClient {
    /// Creates a new `PushinatorClient` instance with the default API base URL.
    ///
    /// # Arguments
    /// * `api_token` - A string containing the API token for authentication.
    ///
    /// # Returns
    /// A new instance of `PushinatorClient`.
    pub fn new(api_token: String) -> Self {
        PushinatorClient {
            api_token: api_token.to_string(),
            base_url: "https://api.pushinator.com/".to_string(),
        }
    }

    /// Creates a new `PushinatorClient` instance with a custom base URL, typically for testing purposes.
    ///
    /// # Arguments
    /// * `api_token` - A string containing the API token for authentication.
    /// * `mock_url` - A string containing the mock URL for the API.
    ///
    /// # Returns
    /// A new instance of `PushinatorClient`.
    pub fn new_test(api_token: String, mock_url: String) -> Self {
        PushinatorClient {
            api_token: api_token.to_string(),
            base_url: mock_url,
        }
    }

    /// Sends a notification to a specific channel synchronously.
    ///
    /// # Arguments
    /// * `channel_id` - ID of the channel to send the notification to.
    /// * `notification` - notification message.
    ///
    /// # Returns
    /// * `Ok(())` if the notification was sent successfully.
    /// * `Err` containing an error if the operation failed.
    pub fn send_notification_sync(
        &self,
        channel_id: String,
        notification: &str,
    ) -> Result<(), Box<dyn Error>> {
        let api_url = format!("{}/api/v1/send_notification", self.base_url);

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
                status, error_body
            )))
        }
    }

        /// Sends a notification to a specific channel asynchronously.
    ///
    /// # Arguments
    /// * `channel_id` - ID of the channel to send the notification to.
    /// * `notification` - notification message.
    ///
    /// # Returns
    /// * `Ok(())` if the notification was sent successfully.
    /// * `Err` containing an error if the operation failed.
    pub async fn send_notification(
        &self,
        channel_id: String,
        notification: &str,
    ) -> Result<(), Box<dyn Error>> {
        let api_url = format!("{}/api/v1/send_notification", self.base_url);

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
                status, error_body
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};
    use tokio;

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

        let client = PushinatorClient::new_test("test-api-token".to_string(), mockito::server_url());
        let result =
            client.send_notification_sync("test-channel-id".to_string(), "Test notification");

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

        let client = PushinatorClient::new_test("test-api-token".to_string(), mockito::server_url());
        let result =
            client.send_notification_sync("test-channel-id".to_string(), "Test notification");

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

        let client = PushinatorClient::new_test("test-api-token".to_string(), mockito::server_url());
        let result = client
            .send_notification("test-channel-id".to_string(), "Test notification")
            .await;
        println!("{:#?}", result);
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

        let client = PushinatorClient::new_test("test-api-token".to_string(), mockito::server_url());
        let result = client
            .send_notification("test-channel-id".to_string(), "Test notification")
            .await;

        assert!(result.is_err());
    }
}
