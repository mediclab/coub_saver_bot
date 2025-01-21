use crate::Application;
use anyhow::Result;
use regex::Regex;
use std::sync::Arc;
use teloxide::types::{InputFile, MessageKind, User};
use teloxide::{prelude::*, utils::command::BotCommands};

pub struct MessageHandler {
    pub app: Arc<Application>,
    pub msg: Message,
}

impl MessageHandler {
    pub async fn handle(msg: Message, app: Arc<Application>) -> Result<()> {
        let handler = Self { app, msg };

        if let MessageKind::Common(_) = handler.msg.kind {
            if handler.msg.chat.is_private() {
                handler.private().await?;
            }
        }

        Ok(())
    }

    async fn private(&self) -> Result<()> {
        if let Some(url) = self.msg.text() {
            let re = Regex::new(r"^https?://(?:www\.)?coub\.com/view/(?<id>\w{3,10})/?$").unwrap();

            let Some(caps) = re.captures(url) else {
                self.app
                    .bot
                    .send_message(self.msg.chat.id, "❌ Неверная ссылка на Coub!")
                    .await?;
                return Ok(());
            };

            let api_url = format!("https://coub.com/api/v2/coubs/{}", &caps["id"]);

            match self.app.coub_client.get_file_url(api_url).await {
                Some(url) => {
                    self.app
                        .bot
                        .send_video(ChatId(self.app.receiver), InputFile::url(url.parse()?))
                        .caption(format!(
                            "💥 Пользователь {} прислал новый куб!",
                            get_user_text(self.msg.from.as_ref().unwrap())
                        ))
                        .await?;

                    self.app
                        .bot
                        .send_message(
                            self.msg.chat.id,
                            "💥 О, спасибо. Отправил куб на модерацию админу",
                        )
                        .await?;
                }
                None => {
                    self.app
                        .bot
                        .send_message(self.msg.chat.id, "❌ Оппа, произошла какая то ошибка")
                        .await?;
                }
            }
        }

        Ok(())
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Команды которые поддерживает бот:"
)]
pub enum BotCommand {
    #[command(description = "Информация о боте")]
    Help,
    #[command(description = "Старт")]
    Start,
}

pub struct CommandHandler {
    pub app: Arc<Application>,
    pub msg: Message,
}

impl CommandHandler {
    pub async fn handle(msg: Message, cmd: BotCommand, app: Arc<Application>) -> Result<()> {
        let handler = Self { app, msg };

        if !handler.msg.chat.is_private() {
            return Ok(());
        }

        match cmd {
            BotCommand::Help => {
                handler.help().await?;
            }
            BotCommand::Start => {
                handler.start().await?;
            }
        };

        Ok(())
    }

    async fn help(&self) -> Result<()> {
        self.app
            .bot
            .send_message(
                self.msg.chat.id,
                format!("Версия бота: {}", self.app.version),
            )
            .await?;

        Ok(())
    }

    async fn start(&self) -> Result<()> {
        self.app.bot
            .send_message(self.msg.chat.id, "🤟 Привет, дружище!\n\nРад, что ты заглянул!\n\nПрисылай ссылку вида:\nhttps://coub.com/view/#coub_id#\n\nИ я все сделаю в лучшем виде! 👌")
            .await?;

        Ok(())
    }
}

pub fn get_user_text(user: &User) -> String {
    match &user.username {
        Some(uname) => format!("@{uname}"),
        None => format!("<a href=\"{}\">{}</a>", user.url(), user.first_name),
    }
}
