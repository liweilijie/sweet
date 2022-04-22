use chrono::{Local, NaiveDateTime};

pub fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn default_expire() -> NaiveDateTime {
    Local::now().naive_local() + chrono::Duration::days(30)
}
