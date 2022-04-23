use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
use serde_json::Value;
use std::env;
pub async fn request(
    url: &String,
    method: Method,
    body: Value,
    method_name: &str,
) -> Response<Body> {
    let auth_value = format!(
        "Bearer {}",
        env::var("BOT_ACCESS_TOKEN").expect("BOT_ACCESS_TOKEN not found")
    );

    let body_content = Body::from(body.to_string());

    let req = Request::builder()
        .method(method)
        .uri(url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_value)
        .body(body_content)
        .unwrap();
    println!("---{}---", method_name);
    println!("Request: {:?}", req);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let resp = client.request(req).await.unwrap();

    println!("Response: {:?}", resp);
    println!("-------");
    resp
}
