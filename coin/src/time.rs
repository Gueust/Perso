use chrono;
use chrono::TimeZone;

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";
pub const LEN: usize = 29;

#[derive(Clone)]
pub struct Time(chrono::DateTime<chrono::Utc>);

impl Time {
    pub fn to_string(&self) -> String {
        let &Time(ref time) = self;
        time.format(FORMAT).to_string()
    }

    pub fn now() -> Time {
        Time(chrono::Utc::now())
    }

    pub fn parse(str: &str) -> Result<Time, String> {
        chrono::Utc.datetime_from_str(str, FORMAT)
            .map(|e| Time(e))
            .map_err(|e| e.to_string())
    }

    pub fn epoch() -> Time {
        Time(chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(0, 0), chrono::Utc))
    }

    pub fn signed_duration_since(&self, ref_time: &Time) -> chrono::Duration {
        let &Time(ref time) = self;
        let &Time(ref ref_time) = ref_time;
        time.signed_duration_since(*ref_time)
    }
}
