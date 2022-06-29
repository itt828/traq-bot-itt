use anyhow::Result;
use traq::bot::Bot;

pub async fn shellgei() -> Result<()> {
    let re = Regex::new(r"@(?i)bot_itt\sshellgei").unwrap();

    let req_str = re.replace(raw_s, "");
    let client = reqwest::Client::new();
    let body = serde_json::json!({
      "code": req_str,
      "images": [],
    });
    let resp = client
        .post("https://websh.jiro4989.com/api/shellgei")
        .json(&body)
        .send()
        .await?;
    let resp: Value = serde_json::from_str(&resp.text().await?)?;
    let stdout = resp["stdout"].as_str().unwrap();
    let stderr = resp["stderr"].as_str().unwrap();
    //                    let imgs = resp["images"]
    //                        .as_array()
    //                        .unwrap()
    //                        .iter()
    //                        .map(|x| async {
    //                            let s = x.as_str().unwrap();
    //                            let raw_image = base64::decode(s).unwrap();
    //                            let image_id = bot.upload(raw_image, channel_id).await.unwrap().id;
    //                            format!("https://q.trap.jp/files/{}", image_id)
    //                        })
    //                        .collect::<Vec<_>>();

    let imgs = resp["images"].as_array().unwrap();
    if imgs.len() >= 1 {
        let img = &imgs[0];
        let fmt = img["format"].as_str().unwrap();
        let img = img["image"].as_str().unwrap();
        let fmt = {
            match fmt {
                "jpe" | "jpg" => "jpeg",
                x => x,
            }
        };
        let raw_image = base64::decode(img).unwrap();
        let image_id;
        image_id = bot
            .upload(
                raw_image,
                &format!("image/{}", fmt),
                &format!("shellgei.{}", fmt),
                channel_id,
            )
            .await
            .unwrap()
            .id;
        let image_url = format!("https://q.trap.jp/files/{}", image_id);
        let msg = if stdout != "" {
            format!("{}\n{}", stdout, image_url)
        } else {
            format!(
                "### stdout\n{}\n### stderr\n{}\n{}",
                stdout, stderr, image_url
            )
        };
        bot.post_message(channel_id, &msg, false).await?;
    } else {
        let msg = if stdout != "" && stderr == "" {
            format!("{}", stdout)
        } else {
            format!("### stdout\n{}\n### stderr\n{}", stdout, stderr)
        };
        bot.post_message(channel_id, &msg, false).await?;
    }
}
