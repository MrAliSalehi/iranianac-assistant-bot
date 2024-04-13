use teloxide::types::{Message, Update};

pub fn is_start(msg: Message) -> bool {
    msg.text().unwrap().eq("/start")
}

pub fn is_add_new_class(msg: Message) -> bool {
    msg.text().unwrap().starts_with("add")
}

pub fn is_admin(admins: Vec<u64>) -> impl Fn(Update) -> bool {
    move |up: Update| admins.contains(&up.from().unwrap().id.0)
}

pub fn is_mod(mods: Vec<u64>) -> impl Fn(Update) -> bool {
    move |up: Update| mods.contains(&up.from().unwrap().id.0)
}

pub fn is_main_chat(u: Update, main_chat: i64) -> bool {
    u.chat().is_some() && u.chat().unwrap().id.0 == main_chat //.to_string().replace("-100", "").parse::<i64>().unwrap()
}

pub fn is_text(msg: Message) -> bool {
    msg.text().is_some()
}

pub fn is_disable_reminder(msg: Message) -> bool {
    msg.text().unwrap().eq("off")
}

pub fn is_enable_reminder(msg: Message) -> bool {
    msg.text().unwrap().eq("on")
}

pub fn from_valid(msg: Message) -> bool {
    msg.from().is_some()
}

pub fn is_disable_class(msg: Message) -> bool {
    msg.text().unwrap().starts_with("disable")
}

pub fn is_enable_class(msg: Message) -> bool {
    msg.text().unwrap().starts_with("enable")
}

pub fn is_class_list(msg: Message) -> bool {
    msg.text().unwrap().eq("list")
}

pub fn is_upcoming_classes(msg: Message) -> bool {
    msg.text().unwrap().eq("اخیر")
}

pub fn is_date(msg: Message) -> bool {
    let t = msg.text().unwrap().trim();
    t.eq("date") || t.eq("تاریخ") || t.eq("زمان") || t.eq("تایم")
}

pub fn is_pv(u: Update) -> bool {
    u.from().is_some() && u.chat().unwrap().id.0 == (u.from().unwrap().id.0 as i64)
}
