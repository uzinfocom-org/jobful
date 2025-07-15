use crate::functions;
use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", parse_with = "split")]
#[command(description = "These are the commands that I can understand:")]
pub enum Commands {
    /// List existing commands
    Help,

    /// Starting point of the bot
    Start,

    /// About da bot
    About,

    /// Way to contact with HR
    Contact,

    /// Way to check personal ID
    Check,

    /// Publishing replied post
    Publish,
}

pub fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        // Inline
        .branch(Update::filter_inline_query().endpoint(functions::inline))
        // Commands
        .branch(
            Update::filter_message()
                .filter_command::<Commands>()
                .endpoint(functions::commands),
        )
        // Rest
        .branch(Update::filter_message().endpoint(functions::triggerer))
}

pub fn dispatch(
    bot: &Bot,
    deps: DependencyMap,
) -> Dispatcher<Bot, Box<dyn std::error::Error + Send + Sync>, teloxide::dispatching::DefaultKey> {
    Dispatcher::builder(bot.clone(), handler())
        .dependencies(deps) // dptree::deps![topics, pkgs]
        // If no handler succeeded to handle an update, this closure will be called
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {upd:?}");
        })
        // If the dispatcher fails for some reason, execute this handler
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
}
