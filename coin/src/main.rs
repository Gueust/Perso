extern crate ws;
extern crate env_logger;
extern crate serde;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::cell::RefCell;
use std::collections::BTreeMap;

enum Side {
    Buy,
    Sell,
}

fn side_of_str(str : &str) -> Option<Side> {
    if str == "buy" { Some(Side::Buy) }
    else if str == "sell" { Some(Side::Sell) }
    else { None }
}

struct BookProcessor {
    bid_sizes : BTreeMap< i64, f64 >,
    ask_sizes : BTreeMap< i64, f64 >,
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

    fn update(&mut self, side: Side, price: i64, size: f64) {
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

    fn parse_price(s: &str) -> Result<i64, String> {
        let res: Result<f64, _> = s.parse();
        let res = res.map_err(|e| e.to_string());
        Ok((res? * 1_000_000.0) as i64)
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

    fn on_message(&self, msg: &str) -> Result<(), String> {
        let json: serde_json::Value = serde_json::from_str(&msg)
            .map_err(|e| e.to_string())?;
        match self.message_type(&json)? {
            MessageType::Error => {
                let error: Error = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                println!("error: {:?}", error)
            },
            MessageType::L2update => {
                let l2update: L2update = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                let mut book_processor = self.book_processor.borrow_mut();
                for &(ref side, ref price, ref size) in l2update.changes.iter() {
                    let price = JsonProcessor::parse_price(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    let side = match side_of_str(side) {
                        Some(side) => side,
                        None => Err(format!("unable to parse side {}", side))?
                    };
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
                    let price = JsonProcessor::parse_price(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    book_processor.update(Side::Buy, price, size);
                }
                for &(ref price, ref size) in snapshot.asks.iter() {
                    let price = JsonProcessor::parse_price(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    book_processor.update(Side::Sell, price, size);
                }
            },
            MessageType::Subscriptions => {
                let subscriptions: Subscriptions = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                println!("subscriptions: {:?}", subscriptions)
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

fn connect(json_processor: &JsonProcessor, address: &str) -> Result<(), ws::Error> {
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
                    match json_processor.on_message(&msg) {
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

fn main() {
    env_logger::init().unwrap();
    let mut json_processor = JsonProcessor::new();
    connect(&mut json_processor, "wss://ws-feed.gdax.com").unwrap();
    println!("Done!");
}
