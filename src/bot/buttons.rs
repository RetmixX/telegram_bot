use std::path::Path;
use lazy_static::lazy_static;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use crate::config::ConfigApp;
use crate::hardware::camera::human_from_camera;
use crate::hardware::shutdown_device;

lazy_static! {
    static ref USER_ID: ChatId = ConfigApp::load_config().user_id;
}

pub async fn handle_input_buttons(query: CallbackQuery, bot: Bot) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_id = query.from.id;
    let input_button = query.data.unwrap();
    match input_button.as_str() {
        "shutdown" => {
            match user_id.0 == USER_ID.0 as u64 {
                true => {
                    shutdown_device();
                    let _ = bot.send_message(user_id, "Goodbye").await;
                }
                false => {
                    let _ = bot.send_message(user_id, "Sorry dude, 500 from server")
                        .await;
                }
            }
        }
        "check_place" => {
            let path = "check_place.jpg";
            match user_id.0 == USER_ID.0 as u64 {
                true => {
                    human_from_camera(path);
                    let _ = bot.send_photo(user_id,
                                           InputFile::file(Path::new(path)))
                        .reply_markup(load_buttons()).await?;
                }
                false => {
                    let _ = bot.send_message(user_id, "Sorry dude, 500 from server")
                        .await;
                }
            }
        }
        _ => {
            let _ = bot.send_message(user_id, "Unknown command").reply_markup(load_buttons()).await;
        }
    }

    Ok(())
}

pub fn load_buttons() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::default()
        .append_row(vec![
            InlineKeyboardButton::callback("Shutdown", "shutdown"),
            InlineKeyboardButton::callback("Check place", "check_place"),
        ])
}