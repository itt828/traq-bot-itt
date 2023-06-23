use crate::{command::exec_command, Resource};
use std::sync::Arc;
use traq_ws_bot::events::payload;

pub async fn message_created_handler(payload: payload::MessageCreated, resource: Arc<Resource>) {
    exec_command(payload.message.clone(), resource)
        .await
        .unwrap();
}
pub async fn direct_message_created_handler(
    payload: payload::DirectMessageCreated,
    resource: Arc<Resource>,
) {
    exec_command(payload.message.clone(), resource)
        .await
        .unwrap();
}
