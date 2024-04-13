pub use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn init_logger() -> eyre::Result<()> {
    let encoder = Box::new(PatternEncoder::new("{l} {m}\n"));

    let config = if cfg!(not(debug_assertions)) {
        let logfile = FileAppender::builder()
            .encoder(encoder)
            .build("log/output.log")
            .unwrap();
        Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(
                Root::builder()
                    .appender("logfile")
                    .build(log::LevelFilter::Warn),
            )
            .unwrap()
    } else {
        let stdout = ConsoleAppender::builder().encoder(encoder).build();
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                Root::builder()
                    .appender("stdout")
                    .build(log::LevelFilter::Info),
            )
            .unwrap()
    };

    log4rs::init_config(config).unwrap();
    Ok(())
}
