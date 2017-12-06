use std::collections::BTreeMap;

use side::Side;
use price::Price;
use time::Time;

// The current snapshot status, starts with InitialSnapshot and moves to PostSnapshot
// once a non-snapshot update has been received.
enum SnapshotStatus {
    InitialSnapshot,
    PostSnapshot,
    Error, // Received a snapshot update after a non-snapshot one, this cannot be recovered from.
}

impl SnapshotStatus {
    fn update(&self, initial_snapshot: bool) -> SnapshotStatus {
        match self {
            &SnapshotStatus::InitialSnapshot => {
                if initial_snapshot {
                    SnapshotStatus::InitialSnapshot
                } else {
                    SnapshotStatus::PostSnapshot
                }
            },
            &SnapshotStatus::PostSnapshot => {
                if initial_snapshot {
                    SnapshotStatus::Error
                } else {
                    SnapshotStatus::PostSnapshot
                }
            },
            &SnapshotStatus::Error => SnapshotStatus::Error,
        }
    }
}

// The different reasons for which the book data should not be used.
pub enum NotLiveStatus {
    InitialSnapshot,
    SnapshotError,
    Stale,
}

pub struct BookProcessor {
    bid_sizes: BTreeMap< Price, f64 >,
    ask_sizes: BTreeMap< Price, f64 >,
    total_bid_size: f64,
    total_ask_size: f64,
    last_update: Time,
    snapshot_status: SnapshotStatus,
}

// TODO: take the product name as argument + assert that it is correct.
// TODO: on_initial_snapshot_done.
// TODO: on_error (call clear_on_snapshot).
impl BookProcessor {
    pub fn new() -> BookProcessor {
        BookProcessor {
            bid_sizes: BTreeMap::new(),
            ask_sizes: BTreeMap::new(),
            total_bid_size: 0.0,
            total_ask_size: 0.0,
            last_update: Time::epoch(),
            snapshot_status: SnapshotStatus::InitialSnapshot,
        }
    }

    pub fn clear_on_snapshot(&mut self) {
        self.bid_sizes.clear();
        self.ask_sizes.clear();
        self.total_bid_size = 0.0;
        self.total_ask_size = 0.0;
        self.last_update = Time::epoch();
        self.snapshot_status = SnapshotStatus::InitialSnapshot;
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

    pub fn on_update(&mut self, time: &Time, side: Side, price: Price, size: f64, initial_snapshot: bool) {
        self.last_update = time.clone();
        self.snapshot_status = self.snapshot_status.update(initial_snapshot);
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

    pub fn status(&self, time: &Time) -> Result<(), NotLiveStatus> {
        match self.snapshot_status {
            SnapshotStatus::InitialSnapshot => Err(NotLiveStatus::InitialSnapshot),
            SnapshotStatus::Error => Err(NotLiveStatus::SnapshotError),
            SnapshotStatus::PostSnapshot => {
                let time_since_last_update = time.signed_duration_since(&self.last_update);
                if time_since_last_update.num_milliseconds() > 500 {
                    Err(NotLiveStatus::Stale)
                } else {
                    Ok(())
                }
            }
        }
    }
}
