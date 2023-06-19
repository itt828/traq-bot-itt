use crate::GPS_EARTHQUAKE;
use chrono::Local;
use earthquake_info::{eew::get_eew, models::eew::Eew};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::Job;
use traq::{
    apis::{
        configuration::Configuration,
        message_api::{edit_message, post_message},
    },
    models::PostMessageRequest,
};

pub async fn eew_info_cron(cron_expr: &str, config: Arc<Configuration>) -> Job {
    let last_message_id: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    Job::new_async(cron_expr, move |_uuid, _l| {
        let last_message_id_clone = last_message_id.clone();
        let config_clone = config.clone();
        Box::pin(async move {
            let now = Local::now();
            let new_eew = get_eew(now).await.unwrap();
            match &*new_eew.report_num {
                "" => (),
                "1" => {
                    let resp = eew_post_handler(&new_eew, config_clone).await;
                    {
                        let mut last_message_id_locked = last_message_id_clone.lock().unwrap();
                        *last_message_id_locked = Some(resp.id.to_string());
                    }
                }
                _ => {
                    let last_message_id_clone_clone = last_message_id_clone.lock().unwrap().clone();
                    eew_edit_handler(
                        &new_eew,
                        &*last_message_id_clone_clone.unwrap(),
                        config_clone,
                    )
                    .await;
                }
            }
        })
    })
    .unwrap()
}

async fn eew_post_handler(eew: &Eew, config: Arc<Configuration>) -> traq::models::Message {
    let message = format!(r"{:#?}", eew);
    post_message(
        &config,
        GPS_EARTHQUAKE,
        Some(PostMessageRequest {
            content: message,
            embed: None,
        }),
    )
    .await
    .unwrap()
}
async fn eew_edit_handler(eew: &Eew, message_id: &str, config: Arc<Configuration>) {
    let message = format!(r"{:#?}", eew);
    edit_message(
        &config,
        message_id,
        Some(PostMessageRequest {
            content: message,
            embed: None,
        }),
    )
    .await;
}
