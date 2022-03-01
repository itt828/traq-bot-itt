use crate::models::events::system::*;
use http::StatusCode;

pub fn handle_ping(body: Ping) -> StatusCode {
    println!("[{}] PING", body.event_time);
    StatusCode::NO_CONTENT
}
