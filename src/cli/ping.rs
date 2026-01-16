//! Ping command implementation.

use crate::client::RedmineClient;
use crate::error::Result;

/// Execute the ping command.
pub async fn execute(client: &RedmineClient) -> Result<crate::client::endpoints::PingResponse> {
    client.ping().await
}
