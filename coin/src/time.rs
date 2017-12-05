use chrono;
use chrono::TimeZone;

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";
pub const LEN: usize = 29;

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
}
