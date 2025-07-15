#![allow(unused_macros)]
#![allow(unused_imports)]

use crate::utils::{inlines::*, resources::Resources};
use std::error::Error;
use teloxide::{prelude::*, types::*};

macro_rules! return_err_answer {
    ($bot:ident, $q:ident, $title:expr, $msg:expr) => {
        return {
            $bot.answer_inline_query(
                $q.id,
                vec![InlineQueryResultArticle::new(
                    uuid::Uuid::new_v4(),
                    $title,
                    InputMessageContent::Text(
                        InputMessageContentText::new($msg)
                            .parse_mode(ParseMode::Html)
                            .link_preview_options(LinkPreviewOptions {
                                is_disabled: true,
                                url: None,
                                prefer_small_media: false,
                                prefer_large_media: false,
                                show_above_text: false,
                            }),
                    ),
                )
                .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        }
    };
}

pub async fn inline(
    bot: Bot,
    mut resources: Resources,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let parsed: String = q.query.clone();

    match parsed.len() {
        0 => return_err_answer!(bot, q, "Qidirishni boshlang!", NO_INPUT),
        1.. => {}
    }

    if resources.outdated() {
        resources = resources.clone().update().await.unwrap_or(resources);
    }

    let results = resources.search(parsed, 5);

    if results.is_empty() {
        return_err_answer!(bot, q, "Hech narsa topilmadi!", NOT_FOUND)
    }

    let results = results.iter().map(|d| {
        InlineQueryResult::Article(
            InlineQueryResultArticle::new(
                uuid::Uuid::new_v4(),
                d.title.clone(),
                InputMessageContent::Text(
                    InputMessageContentText::new(view_generate(d))
                        .parse_mode(ParseMode::Html)
                        .link_preview_options(LinkPreviewOptions {
                            is_disabled: true,
                            url: None,
                            prefer_small_media: false,
                            prefer_large_media: false,
                            show_above_text: false,
                        }),
                ),
            )
            .description(preview_generate(d))
            .reply_markup(kb_generate(d)),
        )
    });

    bot.answer_inline_query(q.id, results).send().await?;

    Ok(())
}
