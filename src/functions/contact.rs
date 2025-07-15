use crate::private;
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Taklif, yoki bog'lanmoqchimisiz?!</b>

Ushbu keltirilgan tugmalar orqali bizning HR mutaxassislarimiz bilan bog'lanishingiz mumkin.
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
        .url("Shahzoda Nosirova", "https://t.me/shakhzoda_nosirova")
        .unwrap()
}
