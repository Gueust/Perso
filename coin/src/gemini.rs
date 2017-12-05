use serde_json;

use std::cell::RefCell;

use book_processor::BookProcessor;
use message_processor::MessageProcessor;
use side::Side;
use price::Price;

#[derive(Debug, Serialize, Deserialize)]
struct Change {
    reason: String,
    price: String,
    delta: String,
    remaining: String,
    side: String,
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

    fn get_type(json: &serde_json::Value) -> Result<&str, String> {
        match json {
            &serde_json::Value::Object(ref map) => {
                match map.get("type") {
                    Some(&serde_json::Value::String(ref message_type)) => Ok(message_type),
                    Some(_) => Err("json message has unexpected type".to_string()),
                    None => Err("json message has missing type".to_string()),
                }
            }
            _ => Err("json message is not an object".to_string()),
        }
    }

    fn get_events(&self, json: serde_json::Value) -> Result<Vec<Change>, String> {
        // Remove this once NLL is in.
        {
            let message_type = JsonProcessor::get_type(&json)?;
            if JsonProcessor::get_type(&json)? != "update" {
                Err(format!("unexpected type {}", message_type))?
            }
        }
        let events = match json {
            // We remove the 'events' key in order to avoid cloning the array.
            serde_json::Value::Object(mut map) => map.remove("events"),
            _ => Err("json message is not an object".to_string())?,
        };
        let mut events = match events {
            None => Err("no events")?,
            Some(serde_json::Value::Array(events)) => events,
            Some(_) => Err("unexpected event type")?,
        };
        let mut results = Vec::new();
        for event in events.drain(..) {
            let event_type_is_change = JsonProcessor::get_type(&event)? == "change";
            // TODO: handle the other event types
            if event_type_is_change {
                let change: Change = serde_json::from_value(event)
                    .map_err(|e| e.to_string())?;
                results.push(change);
            }
        }
        Ok(results)
    }

    pub fn subscribe_message() -> Option<String> {
        None
    }
}

impl MessageProcessor for JsonProcessor {
    fn subscribe_message(&self) -> Option<String> {
        JsonProcessor::subscribe_message()
    }

    fn on_message(&self, msg: &str) -> Result<(), String> {
        let json: serde_json::Value = serde_json::from_str(&msg)
            .map_err(|e| e.to_string())?;
        let mut book_processor = self.book_processor.borrow_mut();
        for event in self.get_events(json)? {
            let _initial = event.reason == "initial"; // TODO: handle this
            let side = match event.side.as_str() {
                "bid" => Side::Buy,
                "ask" => Side::Sell,
                _ => Err(format!("unexpected side {}", event.side))?,
            };
            let price = Price::parse_str(&event.price)?;
            let size = JsonProcessor::parse_size(&event.remaining)?;
            book_processor.on_update(side, price, size)
        }
        book_processor.log_summary();
        Ok(())
    }
}
