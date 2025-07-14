pub mod about;
pub mod check;
pub mod contact;
pub mod help;
pub mod inline;
pub mod start;

pub use inline::inline;

use crate::bot::Command;
use crate::functions;
use std::error::Error;
use teloxide::{prelude::*, types::*};

pub async fn commands(
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: Command,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let _ = match cmd {
        Command::Start => functions::start::command(&bot, &msg).await,
        Command::Help => functions::help::command(&bot, &msg, &cmd).await,
        Command::About => functions::about::command(&bot, &msg).await,
        Command::Contact => functions::contact::command(&bot, &msg).await,
        Command::Check => functions::check::command(&bot, &msg).await,
    };

    Ok(())
}

pub async fn triggers(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(thread) = msg.thread_id {
        if msg.chat.id.0 == -1001174263940 && thread.0.0 == 178654 {
            // Delete anything except image
            if msg.photo().is_some() || msg.document().is_some() {
                return Ok(());
            }

            // Yup, ditch it
            return match bot.delete_message(msg.chat.id, msg.id).await {
                Ok(_) => Ok(()),
                Err(_) => Ok(()),
            };
        }
    }

    if let Some(ref user) = msg.from {
        if let Some(username) = user.username.clone() {
            if username == "Channel_Bot" {
                // Try to delete message and ignore error
                match bot.delete_message(msg.chat.id, msg.id).await {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }

    // if let Some(new_chat_members) = msg.new_chat_members() {
    //     let bot_id = bot.get_me().send().await?.id;

    //     if !new_chat_members.iter().any(|user| user.id == bot_id)
    //         && (msg.chat.is_supergroup() || msg.chat.is_group())
    //     {
    //         crate::functions::trigger(&bot, &msg).await?;
    //     }
    // }

    Ok(())
}
