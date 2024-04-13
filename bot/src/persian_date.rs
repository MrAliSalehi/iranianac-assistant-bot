use chrono::DateTime;
use chrono::Datelike;
use chrono::FixedOffset;
use chrono::Weekday;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

pub struct PersianDateService {
    arc_inner: Arc<InnerPersianDateService>,
}

impl Clone for PersianDateService {
    fn clone(&self) -> PersianDateService {
        PersianDateService {
            arc_inner: self.arc_inner.clone(),
        }
    }
}

pub struct InnerPersianDateService {
    client: Client,
    addr: String,
    iran_offset: FixedOffset,
}

impl PersianDateService {
    pub fn now(&self) -> DateTime<FixedOffset> {
        chrono::Local::now().with_timezone(&self.arc_inner.iran_offset)
    }

    pub fn weekday(&self) -> Weekday {
        self.now().weekday()
    }

    pub fn new() -> Self {
        Self {
            arc_inner: Arc::new(InnerPersianDateService {
                client: reqwest::ClientBuilder
                    ::new()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap(),
                addr: String::from("https://api.keybit.ir/time/"),
                iran_offset: chrono::FixedOffset::east_opt(3 * 3600 + 1800).unwrap(),
            }),
        }
    }
    pub async fn get_persian_date(&mut self) -> eyre::Result<PersianDateTime> {
        //let now = chrono::Utc::now().time().format("%H_%M").to_string();

        /* if let Ok(cache) = self.redis.get::<String, String>(now.clone()).await {
            return Ok(serde_json::from_str(&cache)?);
        } */

        let data = self.arc_inner.client.get(&self.arc_inner.addr).send().await?;

        /* let raw = std::str::from_utf8(&data.bytes().await?)?.to_string();

        let obj: PersianDateTime = serde_json::from_str(&raw)?; */

        //self.redis.set(now, raw).await?;
        Ok(data.json::<PersianDateTime>().await?)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersianDateTime {
    time24: PersianTime,
    date: PersianDate,
}

impl PersianDateTime {
    pub fn time(&self) -> String {
        let spl = self.time24.full.en.split(':').collect::<Vec<&str>>();
        format!("{}:{}", spl[0], spl[1])
    }
    pub fn date(&self) -> &String {
        &self.date.full.official.iso.en
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersianTime {
    pub full: FullTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersianDate {
    pub full: FullDate,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullDate {
    pub official: Official,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Official {
    pub iso: Iso,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iso {
    pub en: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTime {
    pub en: String,
}

unsafe impl Send for PersianDateService {}
unsafe impl Sync for PersianDateService {}

unsafe impl Send for InnerPersianDateService {}
unsafe impl Sync for InnerPersianDateService {}
