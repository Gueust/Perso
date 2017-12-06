use serde_json;

use std::cell::RefCell;

use book_processor::BookProcessor;
use message_processor::MessageProcessor;
use side::Side;
use price::Price;
use time::Time;

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

pub struct JsonProcessor {
    // Assumes a single product for now.
    book_processor: RefCell<BookProcessor>,
}

impl JsonProcessor {
    pub fn new() -> JsonProcessor {
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
    fn subscribe_message(&self) -> Option<String> {
        Some(String::from(r#"{"type": "subscribe", "product_ids": ["BTC-USD"], "channels": ["level2", "heartbeat"]}"#))
    }

    fn server_name(&self) -> String {
        "wss://ws-feed.gdax.com".to_string()
    }

    fn on_message(&self, time: &Time, msg: &str) -> Result<(), String> {
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
                    book_processor.on_update(side, price, size)
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
                    book_processor.on_update(Side::Buy, price, size);
                }
                for &(ref price, ref size) in snapshot.asks.iter() {
                    let price = Price::parse_str(price)?;
                    let size = JsonProcessor::parse_size(size)?;
                    book_processor.on_update(Side::Sell, price, size);
                }
            },
            MessageType::Subscriptions => {
                let subscriptions: Subscriptions = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                info!("subscriptions: {:?}", subscriptions)
            },
            MessageType::Heartbeat => {
                let _heartbeat: Heartbeat = serde_json::from_value(json)
                    .map_err(|e| e.to_string())?;
                self.book_processor.borrow().log_summary();
            },
        }
        Ok(())
    }
}


