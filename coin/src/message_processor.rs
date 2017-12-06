use std;
use std::cell::RefCell;
use std::io::Write;
use std::fs::File;
use time;

enum LoggerKind {
    File(RefCell<File>),
    Stdout,
}

pub struct Logger {
    kind: LoggerKind,
    subscribe_message: Option<String>,
    server_name: String,
}

// TODO: split this into two traits: MessageProcesor and JsonConnection
pub trait MessageProcessor {
    fn server_name(&self) -> String;
    fn subscribe_message(&self) -> Option<String>;
    fn on_message(&self, &time::Time, &str) -> Result<(), String>;

    fn logger(&self, filename: &str) -> Result<Logger, std::io::Error> {
        let kind =
            if filename == "stdout" {
                LoggerKind::Stdout
            } else {
                let file = File::create(filename)?;
                LoggerKind::File(RefCell::new(file))
            };
        Ok(Logger {
            kind: kind,
            subscribe_message: self.subscribe_message(),
            server_name: self.server_name(),
        })
    }
}

// This makes it possible to create a logger on Logger, not sure how useful this would be.
impl MessageProcessor for Logger {
    fn subscribe_message(&self) -> Option<String> {
        self.subscribe_message.clone()
    }

    fn server_name(&self) -> String {
        self.server_name.clone()
    }

    fn on_message(&self, now: &time::Time, message: &str) -> Result<(), String> {
        match self.kind {
            LoggerKind::File(ref file) => {
                file.borrow_mut().write_all(now.to_string().as_bytes())
                    .map_err(|e| e.to_string())?;
                file.borrow_mut().write_all(message.as_bytes())
                    .map_err(|e| e.to_string())?;
                file.borrow_mut().write_all(b"\n")
                    .map_err(|e| e.to_string())?;
            },
            LoggerKind::Stdout => {
                print!("{} {}\n", now.to_string(), message);
            },
        }
        Ok(())
    }
}
