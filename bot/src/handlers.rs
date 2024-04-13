use crate::{
    db, models::class_model::ClassModel, persian_date::PersianDateService, prelude::*,
    reminder::ReminderService, return_if_true,
};
use chrono::Weekday;
use common::{date_time_helpers, info_telegram};
use sqlx::{types::chrono::NaiveTime, PgPool};
use std::{str::FromStr, sync::Arc};
use teloxide::{
    prelude::*,
    types::{ParseMode, ThreadId},
};

pub async fn start(bot: Bot, msg: Message) -> Res {
    bot.send_message(msg.chat.id, "Welcome!").await?;
    Ok(())
}

pub async fn get_date(bot: Bot, msg: Message, mut date_service: PersianDateService) -> Res {
    let reply_id = if msg.thread_id.is_some() {
        msg.thread_id.unwrap().0
    } else {
        msg.id
    };
    let Ok(date) = date_service.get_persian_date().await else {
        bot.send_message(msg.chat.id, "<i>failed to access the date service</i>")
            .parse_mode(ParseMode::Html)
            .reply_to_message_id(reply_id)
            .await?;
        return Ok(());
    };

    bot.send_message(
        msg.chat.id,
        format!("تاریخ: {}\nساعت: {}", date.date(), date.time()),
    )
    .reply_to_message_id(reply_id)
    .await?;

    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn get_upcoming_classes(
    bot: Bot,
    msg: Message,
    db: Arc<PgPool>,
    date_service: PersianDateService,
) -> Res {
    let reply_to = if msg.thread_id.is_some() {
        msg.thread_id.unwrap().0
    } else {
        msg.id
    };
    let classes = db::class_manager::get_all_classes(&db).await?;
    if classes.is_empty() {
        bot.send_message(msg.chat.id, "در حال حاضر هیچ کلاسی وجود ندارد")
            .reply_to_message_id(reply_to)
            .await?;
        return Ok(());
    }
    let weekday = date_service.weekday();
    let todays_classes = classes
        .into_iter()
        .filter(|c| Weekday::from_str(&c.day).unwrap() == weekday)
        .collect::<Vec<ClassModel>>();

    if todays_classes.is_empty() {
        bot.send_message(msg.chat.id, "امروز هیچ کلاسی برگذار نمیشود")
            .reply_to_message_id(reply_to)
            .await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, ClassModel::to_string(todays_classes))
        .reply_to_message_id(reply_to)
        .await?;

    Ok(())
}

pub async fn add_new_class(bot: Bot, msg: Message, db: Arc<PgPool>) -> Res {
    bot.delete_message(msg.chat.id, msg.id).await?;
    //add \n date \n time \n profName \n classname
    let Some(thread_id) = msg.thread_id else {
        return Ok(());
    };

    let spl = msg.text().unwrap().split('\n').collect::<Vec<&str>>();
    return_if_true!(spl.len() < 5);
    let day = date_time_helpers::fr_days_to_eng(spl[1]);
    let Some(day) = day else {
        bot.send_message(msg.chat.id, "<i>unable to parse the day</i>")
            .parse_mode(ParseMode::Html)
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    };

    let time = match NaiveTime::parse_from_str(spl[2], "%H:%M") {
        Ok(time) => time,
        Err(_) => {
            bot.send_message(msg.chat.id, "<i>unable to parse the time</i>")
                .parse_mode(ParseMode::Html)
                .reply_to_message_id(msg.id)
                .await?;
            return Ok(());
        }
    };

    let prof_name = spl[3];
    let class_name = spl[4];

    let class = ClassModel::new(thread_id.0 .0, class_name, prof_name, day, time);
    info_telegram!(format!("new class registered: \n{:#?}", &class));
    let res = db::class_manager::add(&class, &db).await?;

    let t = if res {
        "class registered successfully"
    } else {
        "class already exists!"
    };

    bot.send_message(msg.chat.id, format!("<i>{}</i>", t))
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(thread_id.0)
        .await?;

    Ok(())
}

pub async fn disable_class(bot: Bot, msg: Message, db: Arc<PgPool>) -> Res {
    set_class_status(bot, msg, db, false).await
}

pub async fn enable_class(bot: Bot, msg: Message, db: Arc<PgPool>) -> Res {
    set_class_status(bot, msg, db, true).await
}

pub async fn class_list(bot: Bot, msg: Message, db: Arc<PgPool>) -> Res {
    bot.delete_message(msg.chat.id, msg.id).await?;

    let Some(ThreadId(thread_id)) = msg.thread_id else {
        return Ok(());
    };
    let class_list = db::class_manager::get_all_by_thread_id(&db, thread_id.0).await?;
    if class_list.is_empty() {
        bot.send_message(msg.chat.id, "تا کنون هیچ کلاسی برای این درس ذخیره نشده!")
            .reply_to_message_id(thread_id)
            .parse_mode(ParseMode::Html)
            .await?;

        return Ok(());
    }

    bot.send_message(msg.chat.id, ClassModel::to_string(class_list))
        .reply_to_message_id(thread_id)
        .await?;

    Ok(())
}

pub async fn disable_reminder(bot: Bot, msg: Message, db: Arc<PgPool>) -> Res {
    if !ReminderService::is_enabled() {
        bot.send_message(msg.chat.id, "<i>reminder service is already disabled</i>")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
    }
    ReminderService::disable();
    set_reminder_status(bot, msg, db, false).await
}

pub async fn enable_reminder(
    bot: Bot,
    msg: Message,
    db: Arc<PgPool>,
    date_service: PersianDateService,
) -> Res {
    if ReminderService::is_enabled() {
        bot.send_message(msg.chat.id, "<i>reminder service is already active</i>")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
    }
    ReminderService::enable();

    let ch = msg.chat.id.0;
    set_reminder_status(bot.clone(), msg, Arc::clone(&db), true).await?;
    ReminderService::new(db, bot, ch, &date_service)
        .run()
        .await?;
    Ok(())
}

pub async fn ignore_update() -> Res {
    Ok(())
}

pub async fn invalid_update(bot: Bot, up: Update) -> Res {
    bot.send_message(up.from().unwrap().id, "Invalid command")
        .await?;
    Ok(())
}

async fn set_class_status(bot: Bot, msg: Message, db: Arc<PgPool>, enabled: bool) -> Res {
    bot.delete_message(msg.chat.id, msg.id).await?;

    let Some(ThreadId(thread_id)) = msg.thread_id else {
        return Ok(());
    };

    let spl = msg.text().unwrap().split(' ').collect::<Vec<&str>>();
    return_if_true!(spl.len() < 2);

    let id = spl[1].parse::<i32>();
    return_if_true!(id.is_err());

    let (enabled, c_name, p_name) = db::class_manager::set_status(&db, id?, enabled).await?;

    let stat = lib_common::boolean_to_string(enabled);

    let t = format!("اعلان های کلاس {} با {} اکنون {} شد", c_name, p_name, stat);

    bot.send_message(msg.chat.id, format!("<i>{}</i>", t))
        .reply_to_message_id(thread_id)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn set_reminder_status(bot: Bot, msg: Message, db: Arc<PgPool>, status: bool) -> Res {
    bot.delete_message(msg.chat.id, msg.id).await?;

    db::settings::reminder_status(&db, status).await?;

    bot.send_message(
        msg.chat.id,
        format!(
            "<i>تمامی اعلان ها {} شدند</i>",
            common::boolean_to_string(status)
        ),
    )
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}
