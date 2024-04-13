use chrono::NaiveDateTime;
use lib_common::date_time_helpers;
use sqlx::types::chrono::NaiveTime;

#[derive(Debug)]
pub struct ClassModel {
    pub id: i32,
    pub thread_id: i32,
    pub name: String,
    pub professor_name: String,
    pub day: String,
    pub start_time: NaiveTime,
    pub enabled: bool,
    pub last_notification: Option<NaiveDateTime>,
}

impl ClassModel {
    pub fn new(
        thread_id: i32,
        name: &str,
        professor_name: &str,
        day: String,
        start_time: NaiveTime,
    ) -> ClassModel {
        ClassModel {
            thread_id,
            name: name.to_string(),
            professor_name: professor_name.to_string(),
            day,
            start_time,
            enabled: true,
            id: 0,
            last_notification: None,
        }
    }
    pub fn notification_text(&self) -> String {
        format!(
            "کلاس {} تا دقایقی دیگر ({}) شروع میشود",
            self.professor_name, self.start_time
        )
    }
    pub fn to_string(class_list: Vec<Self>) -> String {
        let f = class_list.first().unwrap();
        let mut t = format!("لیست کلاس های درس {}\n\n", f.name);

        for class in class_list {
            t.push('\n');
            t.push_str(&format!(
                "ایدی : {} (تاپیک شماره {})",
                class.id, class.thread_id
            ));
            t.push('\n');
            t.push_str(&format!("استاد : {}", class.professor_name));
            t.push('\n');
            t.push_str(&format!(
                "روز : {} ({}).",
                &class.day,
                date_time_helpers::eng_days_to_fr(&class.day)
            ));
            t.push('\n');

            t.push_str(&format!("ساعت : {}", class.start_time));
            t.push('\n');

            t.push_str(&format!(
                "اعلان ها : {}",
                lib_common::boolean_to_string(class.enabled)
            ));
            if let Some(ln) = class.last_notification {
                t.push_str(&format!("(اخرین اعلان {})", ln));
            }
            t.push('\n');

            t.push_str("_________");
        }
        t
    }
}
