pub mod logs;

pub mod channel_logger;

pub mod date_time_helpers;

pub use log;

pub async fn startup() -> eyre::Result<()> {
    logs::init_logger()?;
    channel_logger::init_channel_logger().await?;
    Ok(())
}

#[macro_export]
macro_rules! continue_if {
    ($cond:expr) => {
        if $cond {
            continue;
        }
    };
}

pub fn boolean_to_string<'a>(b: bool) -> &'a str {
    if b {
        "فعال"
    } else {
        "غیر فعال"
    }
}
