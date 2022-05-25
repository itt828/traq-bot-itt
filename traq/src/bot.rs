use crate::error::*;
use reqwest::{Body, Client, Method, Response};
use serde_json::Value;

pub struct Bot {
    pub bot_access_token: String,
    pub base_url: String,
}

impl Bot {
    pub async fn api_request_base(
        &self,
        url: &str,
        method: Method,
        body: Value,
    ) -> Result<Response> {
        let auth_value = format!("Bearer {}", self.bot_access_token);
        let body_content = Body::from(body.to_string());
        let client = Client::new();
        let req = client
            .request(method, url)
            .header("Authorization", auth_value)
            .header("Content-Type", "application/json")
            .body(body_content);
        let resp = req.send().await?;
        Ok(resp)
    }
}
