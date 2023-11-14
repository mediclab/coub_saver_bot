use crate::Application;
use std::sync::Arc;
use regex::Regex;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{InputFile, User};

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
                app.bot.send_video(ChatId(app.receiver), InputFile::url(url.parse()?))
                    .caption(format!("💥 Пользователь {} прислал новый куб!", get_user_text(msg.from().unwrap())))
                    .await?;

                app.bot.send_message(msg.chat.id, "💥 О, спасибо. Отправил куб на модерацию админу").await?;
            }
        }

        Ok(())
    }
}

pub fn get_user_text(user: &User) -> String {
    match &user.username {
        Some(uname) => format!("@{uname}"),
        None => format!("<a href=\"{}\">{}</a>", user.url(), user.first_name),
    }
}