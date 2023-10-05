mod bot;
mod config;
mod hardware;

use teloxide::Bot;
use teloxide::prelude::*;
use crate::bot::buttons::handle_input_buttons;
use crate::bot::commands::process_commands;
use crate::bot::send_message;
use crate::config::ConfigApp;

#[tokio::main]
async fn main() {
    let config = ConfigApp::load_config();
    let token_test = config.token;
    let user_id = config.user_id;

    let bot_data = Bot::new(token_test);
    let bot_sender = bot_data.clone();
    let _ = tokio::spawn(async move {
        send_message(bot_sender, user_id).await;
    });
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(process_commands))
        .branch(Update::filter_callback_query().endpoint(handle_input_buttons));

    Dispatcher::builder(bot_data, handler).enable_ctrlc_handler().build().dispatch().await;
}