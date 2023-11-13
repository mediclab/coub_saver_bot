use std::sync::Arc;
use regex::Regex;
use teloxide::types::{InputFile, Message};
use crate::Application;
use anyhow::Result;
use teloxide::prelude::Requester;

pub struct MessageHandler {
    pub app: Arc<Application>,
    pub msg: Message,
}

impl MessageHandler {
    pub async fn handle(msg: Message, app: Arc<Application>) -> Result<()> {
        if let Some(url) = msg.text() {
            let re = Regex::new(r"^https?://(?:www\.)?coub\.com/view/(?<id>\w{6})/?$").unwrap();

            let Some(caps) = re.captures(url) else {
                app.bot.send_message(msg.chat.id, "Неверная ссылка на Coub!").await?;
                return Ok(());
            };

            let api_url = format!("https://coub.com/api/v2/coubs/{}", &caps["id"]);

            if let Some(url) = app.coub_client.get_file_url(api_url).await {
                app.bot.send_video(msg.chat.id, InputFile::url(url.parse()?)).await?;
            }
        }

        Ok(())
    }
}