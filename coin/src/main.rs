extern crate ws;
extern crate env_logger;
extern crate serde;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

trait MessageProcessor {
    fn on_message(&self, &str) -> Result<(), String>;
}

enum Logger {
    File(RefCell<File>),
    Stdout,
}

impl Logger {
    fn new(filename: &str) -> Result<Logger, std::io::Error> {
        if filename == "stdout" {
            Ok(Logger::Stdout)
        } else {
            let file = File::create(filename)?;
            Ok(Logger::File(RefCell::new(file)))
        }
    }
}

impl MessageProcessor for Logger {
    fn on_message(&self, message: &str) -> Result<(), String> {
        match self {
            &Logger::File(ref file) => {
                file.borrow_mut().write_all(message.as_bytes())
                    .map_err(|e| e.to_string())?;
                file.borrow_mut().write_all(b"\n")
                    .map_err(|e| e.to_string())?;
            },
            &Logger::Stdout => {
                print!("{}\n", message);
            },
        }
        Ok(())
    }
}

enum Side {
    Buy,
    Sell,
}

impl Side {
    fn of_str(str : &str) -> Result<Side, String> {
        match str {
            "buy" => Ok(Side::Buy),
            "sell" => Ok(Side::Sell),
            _ => Err(format!("unknown side {}", str)),
        }
    }
}

// Price encoded as int with 6 digits.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Price(i64);

impl Price {
    // Assumes ascii encoding and non-negative prices.
    fn parse_str(str: &str) -> Result<Price, String> {
        let mut pre_dot: i64 = 0;
        let mut seen_dot = false;
        let mut post_dot: i64 = 0;
        let mut post_dot_cnt: i64 = 0;
        for c in str.chars() {
            if c == '.' {
                seen_dot = true;
                continue
            }
            if c < '0' || c > '9' {
                Err(format!("unable to parse as price {}", str))?
            }
            let digit = match c.to_digit(10) {
                Some(digit) => digit as i64,
                None => Err(format!("unable to parse as price {}", str))?
            };
            if seen_dot {
                if post_dot_cnt < 6 {
                    post_dot = 10 * post_dot + digit;
                    post_dot_cnt += 1;
                } else {
                    if digit != 0 {
                        Err(format!("unable to parse as price (too many digits) {}", str))?
                    }
                }
            } else {
                pre_dot = 10 * pre_dot + digit;
            }
        }
        for c in post_dot_cnt..6 {
            post_dot *= 10;
        }
        Ok(Price(pre_dot * 1_000_000 + post_dot))
    }

    fn to_float(&self) -> f64 {
        let &Price(p) = self;
        p as f64 / 1e6
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_float().fmt(f)
    }
}

impl std::fmt::Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_float().fmt(f)
    }
}

struct BookProcessor {
    bid_sizes : BTreeMap< Price, f64 >,
    ask_sizes : BTreeMap< Price, f64 >,
    total_bid_size : f64,
    total_ask_size : f64,
    pre_snapshot: bool,
}

impl BookProcessor {
    fn new() -> BookProcessor {
        BookProcessor {
            bid_sizes: BTreeMap::new(),
            ask_sizes: BTreeMap::new(),
            total_bid_size: 0.0,
            total_ask_size: 0.0,
            pre_snapshot: false,
        }
    }

    fn clear_on_snapshot(&mut self) {
        self.bid_sizes.clear();
        self.ask_sizes.clear();
        self.total_bid_size = 0.0;
        self.total_ask_size = 0.0;
        self.pre_snapshot = false;
    }

    fn log_summary(&self) {
        let best_bid = self.bid_sizes.iter().next_back();
        let best_ask = self.ask_sizes.iter().next();
        info!("bid/ask levels {}/{}: {:?} {:?}",
            self.bid_sizes.len(),
            self.ask_sizes.len(),
            best_bid,
            best_ask);
    }

    fn update(&mut self, side: Side, price: Price, size: f64) {
        if self.pre_snapshot {
            return;
        }
        let ref mut to_update = match side {
            Side::Buy => &mut self.bid_sizes,
            Side::Sell => &mut self.ask_sizes,
        };
        if size == 0.0 {
            to_update.remove(&price);
        } else {
            to_update.insert(price, size);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct L2update {
    product_id: String,
    changes: Vec<(String, String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscriptions {
}

#[derive(Debug, Serialize, Deserialize)]
struct Snapshot {
    product_id: String,
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Heartbeat {
    product_id: String,
    last_trade_id: i64,
    sequence: i64,
    time: String,
}

enum MessageType {
    Error,
    L2update,
    Snapshot,
    Subscriptions,
    Heartbeat,
}

struct JsonProcessor {
    // Assumes a single product for now.
    book_processor: RefCell<BookProcessor>,
}

impl JsonProcessor {
    fn new() -> JsonProcessor {
        JsonProcessor {
            book_processor: RefCell::new(BookProcessor::new()),
        }
    }

    fn parse_size(s: &str) -> Result<f64, String> {
        let res: Result<f64, _> = s.parse();
        res.map_err(|e| e.to_string())
    }

    fn message_type(&self, json: &serde_json::Value) -> Result<MessageType, String> {
        match json {
            &serde_json::Value::Object(ref map) => {
                match map.get("type") {
                    Some(&serde_json::Value::String(ref message_type)) => {
                        match message_type.as_str() {
                            "heartbeat" => Ok(MessageType::Heartbeat),
                            "error" => Ok(MessageType::Error),
                            "l2update" => Ok(MessageType::L2update),
                            "snapshot" => Ok(MessageType::Snapshot),
                            "subscriptions" => Ok(MessageType::Subscriptions),
                            _ => {
                                Err(format!("unexpected type {}", message_type))
                            }
                        }
                    }
                    Some(_) => {
                        Err("json message has unexpected type".to_string())
                    }
                    None => {
                        Err("json message has missing type".to_string())
                    }
                }
            }
            _ => {
                Err("json message is not an object".to_string())
            },
        }
    }
}

impl MessageProcessor for JsonProcessor {
    fn on_message(&self, msg: &str) -> Result<(), String> {
        let json: serde_json::Value = serde_json::from_str(&msg)
            .map_err(|e| e.to_string())?;
        match self.message_type(&json)? {
            MessageType::Error => {
                let error: Error = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                error!("error: {:?}", error)
            },
            MessageType::L2update => {
                let l2update: L2update = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                let mut book_processor = self.book_processor.borrow_mut();
                for &(ref side, ref price, ref size) in l2update.changes.iter() {
                    let price = Price::parse_str(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    let side = Side::of_str(side)?;
                    book_processor.update(side, price, size)
                }
            },
            MessageType::Snapshot => {
                let snapshot: Snapshot = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                info!("processing snapshot");
                let mut book_processor = self.book_processor.borrow_mut();
                book_processor.clear_on_snapshot();
                for &(ref price, ref size) in snapshot.bids.iter() {
                    let price = Price::parse_str(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    book_processor.update(Side::Buy, price, size);
                }
                for &(ref price, ref size) in snapshot.asks.iter() {
                    let price = Price::parse_str(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    book_processor.update(Side::Sell, price, size);
                }
            },
            MessageType::Subscriptions => {
                let subscriptions: Subscriptions = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                info!("subscriptions: {:?}", subscriptions)
            },
            MessageType::Heartbeat => {
                let heartbeat: Heartbeat = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                self.book_processor.borrow().log_summary();
            },
        }
        Ok(())
    }
}

fn connect(processor: &MessageProcessor, address: &str) -> Result<(), ws::Error> {
    let subscribe_message = r#"{"type": "subscribe", "product_ids": ["BTC-USD"], "channels": ["level2", "heartbeat"]}"#;
    ws::connect(address, |out| {
        out.send(subscribe_message).unwrap();
        info!("succesfully sent subscription message");

        move |msg| {
            match msg {
                ws::Message::Binary(vec) => {
                    error!("unexpected binary message {:?}", vec);
                }
                ws::Message::Text(msg) => {
                    match processor.on_message(&msg) {
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

fn replay(processor: &MessageProcessor, filename: &str) -> Result<(), std::io::Error> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line?;
        match processor.on_message(&line) {
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
    if args[1] == "real-time" {
        let mut json_processor = JsonProcessor::new();
        connect(&mut json_processor, "wss://ws-feed.gdax.com").unwrap();
    } else if args[1] == "log" {
        if args.len() <= 2 {
            println!("Usage: {} log filename", args[0]);
            return
        }
        let mut logger = Logger::new(&args[2]).unwrap();
        connect(&mut logger, "wss://ws-feed.gdax.com").unwrap();
    } else if args[1] == "replay" {
        if args.len() <= 2 {
            println!("Usage: {} replay filename", args[0]);
            return
        }
        let mut json_processor = JsonProcessor::new();
        replay(&mut json_processor, &args[2]).unwrap();
    }
}
