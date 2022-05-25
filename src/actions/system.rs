use http::StatusCode;
use traq::models::event::system::*;

pub fn handle_ping(body: Ping) -> StatusCode {
    println!("[{}] PING", body.event_time);
    StatusCode::NO_CONTENT
}
