# pushinator-rust

A Rust crate for sending notifications through the Pushinator API. This library provides both synchronous and asynchronous methods to send notifications.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pushinator = "0.1.0"
```

## Usage

### Creating a Client

Creating a client requires you to have a valid Pushinator API token:

```rust
use pushinator::PushinatorClient;

fn main() {
    let pushinator_client = PushinatorClient::new("PUSHINATOR_API_TOKEN".to_string());
}
```

### Sending Notifications

#### Synchronous Method

```rust
fn main() {
    let pushinator_client = PushinatorClient::new("PUSHINATOR_API_TOKEN".to_string());

    match pushinator_client.send_notification_sync("PUSHINATOR_CHANNEL_ID".to_string(), "Pushinator from Rust!") {
        Ok(_) => println!("Notification sent successfully!"),
        Err(err) => eprintln!("Error sending notification: {}", err),
    }
}
```

#### Asynchronous Method

```rust
#[tokio::main]
async fn main() {
    let pushinator_client = PushinatorClient::new("PUSHINATOR_API_TOKEN".to_string());

    match pushinator_client.send_notification("PUSHINATOR_CHANNEL_ID".to_string(), "Pushinator from Rust!").await {
        Ok(_) => println!("Notification sent successfully!"),
        Err(err) => eprintln!("Error sending notification: {}", err),
    }
}
```


## Steps to Get Started

1. Create a [Pushinator](https://pushinator.com) account
2. Set up a notification channel and save the Channel ID and API token
3. Download the app from the [App Store](https://apps.apple.com/us/app/pushinator/id6477758210) or [Google Play](https://play.google.com/store/apps/details?id=com.apprikos.pushinator) to all devices you want to get notifications on
4. Use the QR code in the dashboard to subscribe your devices to the channel
5. Use the examples above to set up and trigger notifications
