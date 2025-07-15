use crate::bot::Commands;
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &[(&str, &str)] = &[
    ("help", "shu habarni ko'rsatish"),
    ("about", "shu bot haqida ko'proq"),
    ("contact", "HR mutaxassislar bilan ulanish"),
    ("check", "dasturchiga kerakli ma'lumotlar"),
    ("publish", "berilgan habarni chop etish"),
];

pub async fn command(bot: &Bot, msg: &Message, cmd: &Commands) -> ResponseResult<()> {
    let mut text = String::new();

    text.push_str("<b>Hozida faqat ushbu buyruqlar mavjud:</b>\n\n");

    TEXT.iter().for_each(|(command, message)| {
        text.push('/');
        text.push_str(command);
        text.push_str(" - ");
        text.push_str(format!("<code>{message}</code>").as_str());
        text.push('\n');
    });

    text.push('\n');
    text.push_str("<b>Hamda, agar siz vakansiya qidirib ushbu botga yuzlanayotgan bo'lsangiz, iltimos, pastdagi \"Vakansiya\" tugmasini bosing.</b>");

    bot.send_message_tf(msg.chat.id, text, msg)
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
