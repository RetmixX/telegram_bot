pub mod buttons;
pub mod commands;

use std::path::Path;
use std::time::Duration;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use crate::hardware::device::handle_device_input;
use crate::hardware::{send_message_box, shutdown_device};
use online;
use crate::bot::buttons::load_buttons;

pub async fn send_message(bot: Bot, chat_id: ChatId) {
    let path = "intruder.jpg";
    let result = handle_device_input(path);
    if result {
        match online::check(None) {
            Ok(_) => {
                let _ = bot.send_photo(chat_id, InputFile::file(Path::new(path))).await;
                let _ = bot.send_message(chat_id, "Intruder!").reply_markup(load_buttons()).await;
            }
            Err(_) => {
                send_message_box("--warning", "А типо ты самый умный, да?");
                tokio::time::sleep(Duration::from_secs(3)).await;
                shutdown_device();
            }
        }
    }
}