extern crate chrono;
extern crate ws;
extern crate env_logger;

#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::cell::RefCell;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

mod side;
mod price;
mod book_processor;
mod message_processor;
use message_processor::MessageProcessor;
mod gdax;
mod gemini;
mod time;

enum LoggerKind {
    File(RefCell<File>),
    Stdout,
}

struct Logger {
    kind: LoggerKind,
    subscribe_message: Option<String>,
}

impl Logger {
    fn new(filename: &str, subscribe_message: Option<String>) -> Result<Logger, std::io::Error> {
        let kind =
            if filename == "stdout" {
                LoggerKind::Stdout
            } else {
                let file = File::create(filename)?;
                LoggerKind::File(RefCell::new(file))
            };
        Ok(Logger { kind: kind, subscribe_message: subscribe_message })
    }
}

impl MessageProcessor for Logger {
    fn subscribe_message(&self) -> Option<String> {
        self.subscribe_message.clone()
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

fn connect(processor: &MessageProcessor, address: &str) -> Result<(), ws::Error> {
    ws::connect(address, |out| {
        processor.subscribe_message().iter().for_each(|message| {
            out.send(&message[..]).unwrap();
            info!("succesfully sent subscription message");
        });

        move |msg| {
            match msg {
                ws::Message::Binary(vec) => {
                    error!("unexpected binary message {:?}", vec);
                }
                ws::Message::Text(msg) => {
                    let now = time::Time::now();
                    match processor.on_message(&now, &msg) {
                        Ok(()) => {
                        }
                        Err(error) => {
                            error!("json parsing error {} {}", error, msg);
                        }
                    }
                }
            };
            Ok(())
        }
    })
}

fn replay(processor: &MessageProcessor, filename: &str) -> Result<(), String> {
    let file = File::open(filename)
        .map_err(|e| e.to_string())?;
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let now = time::Time::parse(&line[..time::LEN])?;
        match processor.on_message(&now, &line[time::LEN..]) {
            Ok(()) => (),
            Err(e) => error!("Error when parsing message {}", e),
        }
    }
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} real-time|log|replay", args[0]);
        return
    }
    if args[1] == "real-time-gdax" {
        let mut json_processor = gdax::JsonProcessor::new();
        connect(&mut json_processor, "wss://ws-feed.gdax.com").unwrap();
    } else if args[1] == "real-time-gemini" {
        let mut json_processor = gemini::JsonProcessor::new();
        connect(&mut json_processor, "wss://api.gemini.com/v1/marketdata/btcusd").unwrap();
    } else if args[1] == "log-gdax" {
        if args.len() <= 2 {
            println!("Usage: {} log filename", args[0]);
            return
        }
        let mut logger = Logger::new(&args[2], gdax::JsonProcessor::subscribe_message()).unwrap();
        connect(&mut logger, "wss://ws-feed.gdax.com").unwrap();
    } else if args[1] == "replay-gdax" {
        if args.len() <= 2 {
            println!("Usage: {} replay filename", args[0]);
            return
        }
        let mut json_processor = gdax::JsonProcessor::new();
        replay(&mut json_processor, &args[2]).unwrap();
    }
}
