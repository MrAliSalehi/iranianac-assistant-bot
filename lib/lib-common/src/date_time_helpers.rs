pub fn fr_days_to_eng(s: &str) -> Option<String> {
    let r = match s.trim() {
        "شنبه" => "Saturday",
        "یک" => "Sunday",
        "دو" => "Monday",
        "سه" => "Tuesday",
        "چهار" => "Wednesday",
        "پنج" => "Thursday",
        "جمعه" => "Friday",
        _ => "",
    };
    if r.is_empty() {
        None
    } else {
        Some(String::from(r))
    }
}

pub fn eng_days_to_fr(s: &str) -> String {
    String::from(match s {
        "Saturday" => "شنبه",
        "Sunday" => "یک شنبه",
        "Monday" => "دو شنبه",
        "Tuesday" => "سه شنبه",
        "Wednesday" => "چهار شنبه",
        "Thursday" => "پنج شنبه",
        "Friday" => "جمعه",
        _ => "",
    })
}
