use crate::check_reply;
use crate::utils::resources::prelude::Chat as RChat;
use crate::{check_moderator, check_private, utils::resources::Resources};
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{prelude::*, types::*};

// ============================================= //
//               Command Handler                 //
// ============================================= //

static TEXT: &str = r#"
<b>Qaysi hamjamiyatlarga jo'natishni xohlagan bo'lardingiz?</b>

Vakansiya yo'nalishiga qarab keltirilgan hamjamitarlardan birini tanlang va shu hamjamiyatga ushbu vakansiyani yo'naltirib yuboramiz.
"#;

pub async fn command(bot: &Bot, msg: &Message, res: &Resources) -> ResponseResult<()> {
    // Check if it's privat chat
    check_private!(bot, msg);

    // Check if it's one of administrator
    check_moderator!(bot, msg, res);

    // Check if message has reply
    check_reply!(bot, msg);

    let groups = res.groups();

    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard(msg, groups))
        .await?;

    Ok(())
}

pub fn keyboard(message: &Message, groups: &[RChat]) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    let replied_message_id = message.reply_to_message().unwrap().id;

    for (index, group) in groups.iter().enumerate() {
        if index % 3 == 0 {
            keyboard.row();
        }

        // Params Instruction
        // 1 - From ChatID
        // 2 - Message ID
        // 3 - To ChatID
        // 4 - Thread ID
        keyboard.text(
            group.clone().name(),
            &format!(
                "publish_{}_{}_{}_{}",
                message.chat.id,
                replied_message_id,
                group.chat_id().clone(),
                group.thread_id().clone()
            ),
        );
    }

    keyboard.get()
}

// ============================================= //
//               Callback Handler                //
// ============================================= //

pub async fn callback(bot: &Bot, q: &CallbackQuery, resources: &Resources) -> ResponseResult<()> {
    let message = match &q.message {
        Some(m) => m,
        None => return Ok(()),
    };

    let params = match q.data.clone() {
        Some(d) => d,
        None => return Ok(()),
    };

    let params = params
        .split('_')
        .filter(|s| s != &"publish")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let [from_chat_id, from_message_id, to_chat_id, thread_id] = params.iter().as_slice() else {
        return Ok(());
    };

    let from_chat_id = ChatId(from_chat_id.parse().unwrap());
    let to_chat_id = ChatId(to_chat_id.parse().unwrap());
    let from_message_id = MessageId(from_message_id.parse().unwrap());
    let thread_id = ThreadId(MessageId(thread_id.parse().unwrap()));

    let result = bot
        .forward_message_tf(from_chat_id, to_chat_id, from_message_id, &thread_id)
        .await?;

    if let Some(id) = q.inline_message_id.clone() {
        bot.edit_message_text_inline(id, "Muvaffaqiyatli jo'natildi!...")
            .await?;
    }

    Ok(())
}
