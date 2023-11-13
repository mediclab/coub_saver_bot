#[macro_use]
extern crate log;

use std::sync::Arc;
use dotenv::dotenv;
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::bot::MessageHandler;
use crate::coub::CoubClient;

mod bot;
mod coub;

pub struct Application {
    coub_client: CoubClient,
    bot: DefaultParseMode<Bot>,
    receiver: i64,
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

    let app = Arc::new(Application::new());

    info!("Starting dispatch...");

    Dispatcher::builder(app.bot.clone(), dptree::entry()
        .branch(Update::filter_message().endpoint(MessageHandler::handle)))
        .dependencies(dptree::deps![Arc::clone(&app)])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    info!("Good Bye!");
}
