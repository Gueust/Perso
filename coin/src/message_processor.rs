use time;

pub trait MessageProcessor {
    fn subscribe_message(&self) -> Option<String>;
    fn on_message(&self, &time::Time, &str) -> Result<(), String>;
}
