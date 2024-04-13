use dotenv::var;
use lazy_static::lazy_static;
use teloxide::{prelude::*, Bot};
use tokio::sync::Mutex;

lazy_static! {
    pub static ref LOG_CHANNEL: Mutex<i64> = Mutex::new(0);
    pub static ref LOGGER_BOT: Mutex<Option<Bot>> = Mutex::new(None);
}

pub async fn init_channel_logger() -> eyre::Result<()> {
    let mut log_channel = LOG_CHANNEL.lock().await;
    *log_channel = var("LOG_CHANNEL")?.parse::<i64>()?;
    let mut logger_bot = LOGGER_BOT.lock().await;
    *logger_bot = Some(Bot::new(var("LOG_BOT_TOKEN")?));

    Ok(())
}

pub async fn log_message(bot: &Bot, msg: &str) -> eyre::Result<()> {
    let id = *LOG_CHANNEL.lock().await;

    bot.send_message(ChatId(id), msg).await?;
    Ok(())
}

#[macro_export]
macro_rules! log_telegram {
    ($lvl:literal, $msg:expr) => {
        lib_common::channel_logger::log_message(
            lib_common::channel_logger::LOGGER_BOT
                .lock()
                .await
                .as_ref()
                .unwrap(),
            &format!("{} - {}", &$lvl, &$msg),
        )
        .await?
    };
}
#[macro_export]
macro_rules! warn_telegram {
    ($msg:expr) => {
        lib_common::log_telegram!("WARN", $msg);
    };
}

#[macro_export]
macro_rules! info_telegram {
    ($msg:expr) => {
        lib_common::log_telegram!("INFO", $msg);
    };
}
#[macro_export]
macro_rules! err_telegram {
    ($msg:expr) => {
        lib_common::log_telegram!("ERROR", $msg);
    };
}
