#[derive(Debug, Clone)]
pub struct Coin {
    pub symbol: String,
    pub price: String,
}

impl Coin {
    pub fn new(symbol: &str, price: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            price: price.to_string(),
        }
    }
}
