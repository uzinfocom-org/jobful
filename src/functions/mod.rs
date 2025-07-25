pub mod about;
pub mod check;
pub mod contact;
pub mod help;
pub mod inline;
pub mod publish;
pub mod start;

pub use inline::inline;
use orzklv::telegram::topic::Topics;

use crate::functions;
use crate::{bot::Commands, utils::resources::Resources};
use std::error::Error;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::*};

pub async fn commands(
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: Commands,
    res: Resources,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let _ = match cmd {
        Commands::Start => functions::start::command(&bot, &msg).await,
        Commands::Help => functions::help::command(&bot, &msg, &cmd).await,
        Commands::About => functions::about::command(&bot, &msg).await,
        Commands::Contact => functions::contact::command(&bot, &msg).await,
        Commands::Check => functions::check::command(&bot, &msg).await,
        Commands::Publish => functions::publish::command(&bot, &msg, &res).await,
    };

    Ok(())
}

pub async fn callback(
    bot: Bot,
    q: CallbackQuery,
    resources: Resources,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(data) = q.data.clone() {
        let mut args: Vec<&str> = if data.contains('_') {
            data.split('_').collect()
        } else {
            vec![&data]
        };

        let _ = match args.remove(0) {
            "publish" => publish::callback(&bot, &q, &resources).await,
            _ => Ok(()),
        };
    }

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
