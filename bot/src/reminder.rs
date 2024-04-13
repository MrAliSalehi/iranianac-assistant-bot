use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::{db, persian_date::PersianDateService, ArcPg, Res};
use chrono::{Datelike, Weekday};
use lazy_static::lazy_static;
use lib_common::{continue_if, info_telegram};
use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{ChatId, MessageId, ParseMode, Recipient},
    Bot,
};
use tokio::time::sleep;

lazy_static! {
    static ref ENABLED: AtomicBool = AtomicBool::new(true);
}

pub struct ReminderService {
    db: ArcPg,
    bot: Bot,
    main_chat: i64,
    date_service: PersianDateService,
}

impl ReminderService {
    pub fn new(
        db: ArcPg,
        bot: Bot,
        main_chat: i64,
        date_service: &PersianDateService,
    ) -> ReminderService {
        Self {
            db,
            bot,
            main_chat,
            date_service: date_service.clone(),
        }
    }

    pub async fn run(&self) -> Res {
        let db = Arc::clone(&self.db);
        let bot = self.bot.clone();
        let ch = self.main_chat;
        let date_service = self.date_service.clone();
        tokio::spawn(async move {
            Self::enable();
            info_telegram!("reminder service running.");
            Self::notify_classes(db, bot, ch, &date_service).await?;
            info_telegram!("reminder service closed.");
            eyre::Result::<()>::Ok(())
        });
        Ok(())
    }

    pub fn enable() {
        ENABLED
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(true))
            .unwrap();
    }

    pub fn disable() {
        ENABLED
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(false))
            .unwrap();
    }

    pub fn is_enabled() -> bool {
        ENABLED.fetch_or(false, Ordering::Relaxed)
    }
}

impl ReminderService {
    async fn notify_classes(
        db: ArcPg,
        bot: Bot,
        main_chat: i64,
        date_service: &PersianDateService,
    ) -> Res {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let reminder_enabled = db::settings::reminder_enabled(&db).await?;
            if !reminder_enabled {
                info_telegram!("reminder is disabled, aborting...");
                break;
            }

            let iran_dt = date_service.now();

            let today = iran_dt.date_naive().weekday();

            let class_list = db::class_manager::get_classes_for_notification(&db).await?;

            continue_if!(class_list.is_empty());
            for class in class_list {
                let Ok(class_day) = Weekday::from_str(&class.day) else {
                    continue;
                };
                continue_if!(today != class_day);

                let now = iran_dt.time();

                continue_if!(now > class.start_time);

                let diff = (now - class.start_time).abs();

                continue_if!(diff > chrono::Duration::minutes(11));

                let ch = Recipient::Id(ChatId(main_chat));
                let msg = bot
                    .send_message(ch.clone(), format!("<b>{}</b>", class.notification_text()))
                    .disable_notification(false)
                    .parse_mode(ParseMode::Html)
                    .reply_to_message_id(MessageId(class.thread_id))
                    .await?;

                sleep(Duration::from_secs(1)).await;

                bot.pin_chat_message(ch, msg.id).await?;

                db::class_manager::mark_notification(&db, class.id).await?;

                sleep(Duration::from_secs(1)).await;
            }
        }
        Self::disable();
        Ok(())
    }
}
