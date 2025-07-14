pub mod about;
pub mod check;
pub mod contact;
pub mod help;
pub mod inline;
pub mod start;

pub use inline::inline;
use orzklv::telegram::topic::Topics;

use crate::functions;
use crate::{bot::Command, utils::resources::Resources};
use std::error::Error;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::*};

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

pub async fn triggerer(
    bot: Bot,
    msg: Message,
    resources: Resources,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(ref user) = msg.from {
        if resources.is_admin(&user.id) {
            let _ = bot
                .send_message_tf(msg.chat.chat_id().unwrap(), "<b>Siz adminlar ro'yxatida ekansiz!</b>\n\nHabar tayyor bo'lganda, shu habarga reply qilib /publish yozvoring va qolganida yordamlashib yuboraman!", &msg)
                .parse_mode(ParseMode::Html)
                .await;
        }
    }

    Ok(())
}
