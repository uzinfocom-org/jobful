use orzklv::telegram::{timer::Timer, topic::Topics};
use teloxide::{prelude::*, types::ParseMode};

static TEXT: &str = "⚠️ <b>Vay, reply qilish esdan chiqdi chog'i! Bu komandani ishlatish uchun biron habarni reply qilish kerak...</b>";

pub async fn is_reply(bot: &Bot, msg: &Message) -> ResponseResult<bool> {
    if msg.reply_to_message().is_some() {
        return Ok(true);
    }

    match bot.delete_message(msg.chat.id, msg.id).await {
        Ok(_) => {}
        Err(_) => {}
    };

    let message = bot
        .send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .await?;

    bot.delete_timer(message.chat.id, message.id, 10)
        .await
        .await?;

    Ok(false)
}

#[macro_export]
macro_rules! check_reply {
    ($bot:expr, $msg:expr) => {
        use $crate::hooks::is_reply;

        if !is_reply($bot, $msg).await.unwrap() {
            return Ok(());
        }
    };
}

pub use check_reply;
