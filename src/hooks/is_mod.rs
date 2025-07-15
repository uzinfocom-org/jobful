use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

use orzklv::telegram::{keyboard::Keyboard, timer::Timer, topic::Topics};

static TEXT: &str = "⚠️ <b>Bu komanda faqat botning moderatorlari uchun!</b>\n\nAdministrator yoki moderatorlar bilan bog'lanish uchun /contact buyrug'iga muroojat qiling.";

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Keyboard = Keyboard::new();
    keyboard
        .url("Shaxsiy Chat", "https://t.me/uzinfojobful_bot")
        .unwrap()
}

pub async fn is_moderator(bot: &Bot, msg: &Message, res: &Resources) -> ResponseResult<bool> {
    if let Some(ref user) = msg.from {
        if res.is_admin(&user.id) {
            return Ok(true);
        }
    }

    match bot.delete_message(msg.chat.id, msg.id).await {
        Ok(_) => {}
        Err(_) => {}
    };

    let message = bot
        .send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    bot.delete_timer(message.chat.id, message.id, 10)
        .await
        .await?;

    Ok(false)
}

#[macro_export]
macro_rules! moderator {
    ($bot:expr, $msg:expr, $res:expr) => {
        use $crate::hooks::is_moderator;

        if !is_moderator($bot, $msg, $res).await.unwrap() {
            return Ok(());
        }
    };
}

pub use moderator;

use crate::utils::resources::Resources;
