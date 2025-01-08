use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Japan, Tz};

fn current_time_jp() -> DateTime<Tz> {
    Japan.from_utc_datetime(&Utc::now().naive_utc())
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use super::*;

    #[test]
    #[ignore]
    fn gets_current_time_in_japan() {
        let time = current_time_jp();
        let month = time.month0();
        assert_eq!(month, 0);
    }
}
