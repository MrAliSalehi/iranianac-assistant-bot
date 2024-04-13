use dotenv::var;
use filters::*;
use handlers::*;
pub use prelude::*;
use sqlx::PgPool;
use std::sync::Arc;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    dptree::filter,
};

use crate::{persian_date::PersianDateService, reminder::ReminderService};

mod db;
mod filters;
mod handlers;
mod models;
mod persian_date;
mod prelude;
mod reminder;

#[tokio::main]
async fn main() -> Res {
    dotenv::dotenv().ok();

    common::startup().await?;

    let bot = Bot::new(var("MAIN_BOT_TOKEN")?);

    let admins = prelude::set_admins(&bot).await?;

    let db = Arc::new(PgPool::connect(&var("DATABASE_URL")?).await?);

    let main_chat = var("MAIN_CHAT")?.parse()?;

    /* let redis_manager =
        ConnectionManager::new(redis::Client::open(var("REDIS_SERVER")?).unwrap()).await?; */

    let date_service = PersianDateService::new();

    ReminderService::new(Arc::clone(&db), bot.clone(), main_chat, &date_service)
        .run()
        .await?;

    common::info_telegram!("bot starting");

    Dispatcher::builder(bot, handlers(admins, get_mods(), main_chat))
        .dependencies(dptree::deps![
            InMemStorage::<TelegramState>::new(),
            db,
            date_service
        ])
        .enable_ctrlc_handler()
        .worker_queue_size(120)
        .build()
        .dispatch()
        .await;

    Ok(())
}

pub fn handlers(admins: Vec<u64>, mods: Vec<u64>, main_chat: i64) -> UpdateHandler<eyre::Report> {
    let message_handler = Update::filter_message()
        .filter(from_valid)
        .filter(move |u| is_main_chat(u, main_chat))
        .filter(is_text)
        .branch(filter(is_date).endpoint(get_date))
        .branch(filter(is_upcoming_classes).endpoint(get_upcoming_classes))
        .branch(
            filter(is_mod(mods))
                .branch(filter(is_disable_class).endpoint(disable_class))
                .branch(filter(is_enable_class).endpoint(enable_class))
                .branch(filter(is_class_list).endpoint(class_list)),
        )
        .filter(is_admin(admins))
        .branch(filter(is_start).endpoint(start))
        .branch(filter(is_disable_reminder).endpoint(disable_reminder))
        .branch(filter(is_enable_reminder).endpoint(enable_reminder))
        .branch(filter(is_add_new_class).endpoint(add_new_class));
    //.branch(case![TelegramState::MessageTemplate].endpoint(set_message_template));

    dialogue::enter::<Update, InMemStorage<TelegramState>, TelegramState, _>()
        .branch(message_handler)
        .branch(filter(is_pv).endpoint(invalid_update))
        .branch(dptree::endpoint(ignore_update))
}
