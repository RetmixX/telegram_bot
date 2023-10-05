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
/*
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands bot")]
enum Command {
    #[command(description = "Start app")]
    Start,
    #[command(description = "Send message")]
    SendMessage(String),
}

fn load_buttons() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::default()
        .append_row(vec![
            InlineKeyboardButton::callback("Shutdown", "shutdown"),
            InlineKeyboardButton::callback("Check place", "check_place"),
        ])
}

async fn handle_input_buttons(query: CallbackQuery, bot: Bot) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_id = query.from.id;
    let input_button = query.data.unwrap();
    match input_button.as_str() {
        "shutdown" => {
            shutdown_device();
            bot.send_message(user_id, "Goodbye").await;
        }
        "check_place" => {
            let path = "check_place.jpg";
            human_from_camera(path);
            bot.send_photo(user_id, InputFile::file(Path::new(path))).reply_markup(load_buttons()).await?;
        }
        _ => {
            bot.send_message(user_id, "Unknown command").reply_markup(load_buttons()).await;
        }
    }

    Ok(())
}

fn shutdown_device() {
    std::process::Command::new("shutdown")
        .args(["-h", "now"]).spawn().unwrap();
}

async fn send_message(bot: Bot, chat_id: ChatId) {
    let path = "intruder.jpg";
    let result = handle_rofls(path);
    if result {
        match online::check(None) {
            Ok(_) => {
                bot.send_photo(chat_id, InputFile::file(Path::new(path))).await;
            }
            Err(_) => {
                send_message_box("--warning", "А типо ты самый умный, да?");
                tokio::time::sleep(Duration::from_secs(3)).await;
                shutdown_device();
            }
        }
    }
}

async fn process_commands(bot: Bot, msg: Message, me: Me) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                bot.send_message(msg.chat.id, "Okay, start").reply_markup(load_buttons()).await?;
            }
            Ok(Command::SendMessage(message)) => {
                send_message_box("--question", message.as_str());
                bot.send_message(msg.chat.id, "Message send\nWold you like?")
                    .reply_markup(load_buttons()).await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found").await?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn send_message_box(type_window: &str, message: &str) {
    std::process::Command::new("zenity")
        .args([type_window, "--text", message]).spawn().unwrap();
}

fn human_from_camera(path: &str) {
    let mut camera = VideoCapture::new(0, CAP_ANY).unwrap();
    let mut frame = Mat::default();
    camera.read(&mut frame).unwrap();
    let params = opencv::types::VectorOfi32::new();
    imwrite(path, &frame, &params).unwrap();
}

fn handle_rofls(path: &str) -> bool {
    let mut activate_flag = false;
    let device = DeviceState::new();
    loop {
        let keys = device.get_keys();
        //let d = device.get_mouse();
        if !keys.is_empty() {
            match define_activate_combination(&keys) {
                ActivationFlags::Activate => {
                    activate_flag = true;
                    sleep(Duration::from_secs(2));
                    continue;
                }
                ActivationFlags::Deactivate => {
                    activate_flag = false;
                    sleep(Duration::from_secs(2));
                    continue;
                }
                ActivationFlags::Error => {
                    if activate_flag {
                        human_from_camera(path);
                        return true;
                    }
                }
            }
        }

        sleep(Duration::from_millis(100));
    }
}

fn define_activate_combination(keys_enter: &Vec<Keycode>) -> ActivationFlags {
    return if keys_enter.contains(&Keycode::LAlt) && keys_enter.contains(&Keycode::I) {
        ActivationFlags::Activate
    } else if keys_enter.contains(&Keycode::LAlt) && keys_enter.contains(&Keycode::O) {
        ActivationFlags::Deactivate
    } else {
        ActivationFlags::Error
    };
}

enum ActivationFlags {
    Activate,
    Deactivate,
    Error,
}*/