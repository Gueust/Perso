extern crate chrono;
extern crate ws;
extern crate env_logger;

#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

mod side;
mod price;
mod book_processor;
mod message_processor;
use message_processor::MessageProcessor;
mod gdax;
mod gemini;
mod time;

fn connect(processor: &MessageProcessor) -> Result<(), ws::Error> {
    ws::connect(processor.server_name(), |out| {
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

// This returns a box as the MessageProcessor size is unknown at compile time.
fn feed_processor(feed_name: &str) -> Result<Box<MessageProcessor>, String> {
    match feed_name {
        "gdax" => Ok(Box::new(gdax::JsonProcessor::new())),
        "gemini" => Ok(Box::new(gemini::JsonProcessor::new())),
        _ => Err(format!("unsupported feed {}", feed_name)),
    }
}

fn main() {
    env_logger::init().unwrap();
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} real-time|log|replay", args[0]);
        return
    }
    if args[1] == "real-time" {
        if args.len() != 3 {
            println!("Usage: {} real-time gdax|gemini", args[0]);
            return
        }
        let mut processor = feed_processor(&args[2]).unwrap();
        connect(&mut *processor).unwrap();
    } else if args[1] == "log" {
        if args.len() != 4 {
            println!("Usage: {} log gdax|gemini filename", args[0]);
            return
        }
        let processor = feed_processor(&args[2]).unwrap();
        let mut logger = processor.logger(&args[3]).unwrap();
        connect(&mut logger).unwrap();
    } else if args[1] == "replay" {
        if args.len() != 4 {
            println!("Usage: {} replay gdax|gemini filename", args[0]);
            return
        }
        let mut processor = feed_processor(&args[2]).unwrap();
        replay(&mut *processor, &args[3]).unwrap();
    }
}
