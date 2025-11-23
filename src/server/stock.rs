/// A stock quote with ticker, price, volume and timestamp.
#[derive(Debug, Clone)]
pub struct StockQuote {
    /// Stock symbol.
    pub ticker: String,
    /// Price value.
    pub price: f64,
    /// Trade volume.
    pub volume: u32,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

impl StockQuote {
    /// Converts the quote into a `ticker|price|volume|timestamp` string.
    pub fn to_string(&self) -> String {
        format!("{}|{}|{}|{}", self.ticker, self.price, self.volume, self.timestamp)
    }

    /// Parses a quote from a `ticker|price|volume|timestamp` string.
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() == 4 {
            Some(StockQuote {
                ticker: parts[0].to_string(),
                price: parts[1].parse().ok()?,
                volume: parts[2].parse().ok()?,
                timestamp: parts[3].parse().ok()?,
            })
        } else {
            None
        }
    }

    /// Converts the quote into a byte buffer suitable for UDP sending.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.ticker.as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.price.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.volume.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.timestamp.to_string().as_bytes());
        bytes.push(b'\n');
        bytes
    }
}