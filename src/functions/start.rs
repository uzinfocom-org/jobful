use orzklv::{telegram::keyboard::Keyboard, telegram::topic::Topics};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Assalomu alaykum!</b>

Ushbu botni Uzinfocom HR lari uchun vakansiya tarqatish osonlashtirish maqsadida yaratilagan va muntazam ravishda olib boriladi. Iltimos, ko'proq ma'lumotlar uchun /help buyrug'ini chaqiring.
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard
        .url("Vakansiyalar", "https://uzinfocom.uz/uz/company/career")
        .unwrap()
}
