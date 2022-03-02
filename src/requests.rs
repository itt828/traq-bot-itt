use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::json;
use std::env;

pub async fn post_message(content: String, channel_id: String) {
    let url = format!("https://q.trap.jp/api/v3/channels/{}/messages", channel_id);
    let auth_value = format!(
        "Bearer {}",
        env::var("BOT_ACCESS_TOKEN").expect("BOT_ACCESS_TOKEN not found")
    );

    let body_content = Body::from(
        json!({
          "content": content,
          "embed": true
        })
        .to_string(),
    );

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_value)
        .body(body_content)
        .unwrap();

    println!("---post message---");
    println!("Request: {:?}", req);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let resp = client.request(req).await.unwrap();

    println!("Response: {:?}", resp);
    println!("-------\n");
}
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
