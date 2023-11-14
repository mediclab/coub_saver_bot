#[macro_use]
extern crate log;

use crate::bot::{BotCommand, CommandHandler, MessageHandler};
use crate::coub::CoubClient;
use dotenv::dotenv;
use std::sync::Arc;
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

mod bot;
mod coub;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Application {
    coub_client: CoubClient,
    bot: DefaultParseMode<Bot>,
    receiver: i64,
    version: String,
}

impl Application {
    pub fn new() -> Self {
        Self {
            coub_client: CoubClient::new(),
            bot: Bot::from_env().parse_mode(ParseMode::Html),
            receiver: std::env::var("RECEIVER")
                .expect("Необходимо указать получателя, кому отправлять все кубы!")
                .parse::<i64>()
                .expect("Неверный ID поллучателя кубов!"),
            version: VERSION.to_string(),
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Application::new()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();

    info!("Bot version: {}", VERSION);

    let app = Arc::new(Application::new());

    info!("Starting dispatch...");

    Dispatcher::builder(
        app.bot.clone(),
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<BotCommand>()
                    .endpoint(CommandHandler::handle),
            )
            .branch(Update::filter_message().endpoint(MessageHandler::handle)),
    )
    .dependencies(dptree::deps![Arc::clone(&app)])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    info!("Good Bye!");
}
