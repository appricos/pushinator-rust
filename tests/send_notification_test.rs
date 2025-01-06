use pushinator::PushinatorClient;
use serde_json::json;
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

    let client = PushinatorClient::new_test("test-api-token".to_string(), mockito::server_url());
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
