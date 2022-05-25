use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde_json::json;

impl Bot {
    pub async fn edit_subscription(
        &self,
        channel_id: &str,
        on_users: &Vec<String>,
        off_users: &Vec<String>,
    ) -> Result<()> {
        let url = format!("{}/channels/{}/subscribers", self.base_url, channel_id);
        let body = json!({ "on": on_users,"off":off_users });
        let _ = self.api_request_base(&url, Method::PATCH, body).await?;
        Ok(())
    }
}
