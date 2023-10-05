use teloxide::Bot;
use teloxide::macros::BotCommands as Commands;
use teloxide::prelude::*;
use teloxide::types::Me;
use teloxide::utils::command::BotCommands;
use crate::bot::buttons::load_buttons;
use crate::hardware::send_message_box;

#[derive(Commands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands bot")]
enum Command {
    #[command(description = "Start app")]
    Start,
    #[command(description = "Send message")]
    SendMessage(String),
}

pub async fn process_commands(bot: Bot, msg: Message, me: Me) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        }
    }
    Ok(())
}