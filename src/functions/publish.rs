use crate::{moderator, private, utils::resources::Resources};
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Qaysi hamjamiyatlarga jo'natishni xohlagan bo'lardingiz?</b>

Vakansiya yo'nalishiga qarab keltirilgan hamjamitarlardan birini tanlang va shu hamjamiyatga ushbu vakansiyani yo'naltirib yuboramiz.
"#;

pub async fn command(bot: &Bot, msg: &Message, res: &Resources) -> ResponseResult<()> {
    // Check if it's privat chat
    private!(bot, msg);

    // Check if it's one of administrator
    moderator!(bot, msg, res);

    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard
        .url("Ochiq Havolalar", "https://github.com/uzinfocom-org/jobful")
        .unwrap()
}
