pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn of_str(str : &str) -> Result<Side, String> {
        match str {
            "buy" => Ok(Side::Buy),
            "sell" => Ok(Side::Sell),
            _ => Err(format!("unknown side {}", str)),
        }
    }
}
