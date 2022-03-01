use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::json;
use std::env;

pub async fn post_message(content: String, channel_id: String) {
    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "https://q.trap.jp/api/v3/channels/{}/messages",
            channel_id
        ))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("BOT_ACCESS_TOKEN").expect("BOT_ACCESS_TOKEN not found")
            ),
        )
        .body(Body::from(
            json!({
              "content": content,
              "embed": true
            })
            .to_string(),
        ))
        .unwrap();
    println!("{:?}", req);
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let resp = client.request(req).await.unwrap();
    println!("Response: {}", resp.status());
}
