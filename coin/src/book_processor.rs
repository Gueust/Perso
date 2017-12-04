use std::collections::BTreeMap;

use side::Side;
use price::Price;

pub struct BookProcessor {
    bid_sizes : BTreeMap< Price, f64 >,
    ask_sizes : BTreeMap< Price, f64 >,
    total_bid_size : f64,
    total_ask_size : f64,
    pre_snapshot: bool,
}

impl BookProcessor {
    pub fn new() -> BookProcessor {
        BookProcessor {
            bid_sizes: BTreeMap::new(),
            ask_sizes: BTreeMap::new(),
            total_bid_size: 0.0,
            total_ask_size: 0.0,
            pre_snapshot: false,
        }
    }

    pub fn clear_on_snapshot(&mut self) {
        self.bid_sizes.clear();
        self.ask_sizes.clear();
        self.total_bid_size = 0.0;
        self.total_ask_size = 0.0;
        self.pre_snapshot = false;
    }

    pub fn log_summary(&self) {
        let best_bid = self.bid_sizes.iter().next_back();
        let best_ask = self.ask_sizes.iter().next();
        info!("bid/ask levels {}/{}: {:?} {:?}",
            self.bid_sizes.len(),
            self.ask_sizes.len(),
            best_bid,
            best_ask);
    }

    pub fn update(&mut self, side: Side, price: Price, size: f64) {
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
