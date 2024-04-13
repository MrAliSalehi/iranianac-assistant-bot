use dotenv::var;
pub use lib_common as common;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
pub use teloxide::prelude::*;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    types::{BotCommand, BotCommandScope, Recipient},
};

#[derive(Clone, Default)]
pub enum TelegramState {
    #[default]
    Start,
}

pub type Res = eyre::Result<()>;

pub type ArcPg = Arc<Pool<Postgres>>;

pub type AppState = Dialogue<TelegramState, InMemStorage<TelegramState>>;

pub async fn set_admins(bot: &Bot) -> eyre::Result<Vec<u64>> {
    let admins = var("ADMINS")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let commands = vec![
        BotCommand {
            command: "/gtemplate".to_string(),
            description: "get message template".to_string(),
        },
        BotCommand {
            command: "/stemplate".to_string(),
            description: "set message template".to_string(),
        },
    ];
    common::info_telegram!(format!("Included Admins: {:#?}", &admins));

    for id in &admins {
        bot.set_my_commands(commands.clone())
            .scope(BotCommandScope::Chat {
                chat_id: Recipient::Id(ChatId(*id as i64)),
            })
            .await?;
    }

    Ok(admins)
}

pub fn get_mods() -> Vec<u64> {
    var("MODS")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

#[macro_export]
macro_rules! return_if_true {
    ($cond:expr) => {
        if $cond {
            return Ok(());
        }
    };
}
