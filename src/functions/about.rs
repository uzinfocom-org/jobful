use crate::private;
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Hurmatli foydalanuvchi!</b>

Ushbu bot <a href="https://oss.uzinfocom.uz">Uzinfocom Open Source</a> dasturchilari tomonidan yaratilgan va HR mutaxassislar departamenti bilan birgalikda rivojlantirilib boriladi.

"Ochiq Havolalar" tugmasini bosish orqali botimizning tuzilishi va botning kodlari (havolalari) ni ko'zdan kechirib chiqishingiz mumkin.
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    private!(bot, msg);

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
