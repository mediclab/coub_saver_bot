use std::sync::Arc;
use dotenv::dotenv;
use teloxide::{Bot, dptree};
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::Dispatcher;
use teloxide::types::Update;
use crate::bot::MessageHandler;
use crate::coub::CoubClient;

mod bot;
mod coub;
mod ffmpeg;


pub struct Application {
    coub_client: CoubClient,
    bot: Bot
}

impl Application {
    pub fn new() -> Self {
        Self {
            coub_client: CoubClient::new(),
            bot: Bot::from_env(),
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

    Dispatcher::builder(app.bot.clone(), dptree::entry()
        .branch(
            Update::filter_message().endpoint(MessageHandler::handle),
        ))
        .dependencies(dptree::deps![Arc::clone(&app)])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
