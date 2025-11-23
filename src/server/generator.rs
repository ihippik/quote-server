use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use crate::server::stock::StockQuote;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{random, Rng};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader};
use log::{debug, warn};

/// Generates random stock quotes and distributes them to subscribers.
pub struct QuoteGenerator {
    popular_tickers: Vec<String>,
    available_tickers: Vec<String>,
}

impl QuoteGenerator {
    /// Creates a new generator with a list of popular tickers.
    pub fn new(tickers: Vec<String>) -> Self {
        let all = vec![];
        Self { popular_tickers: tickers, available_tickers: all }
    }

    /// Loads available tickers from `tickers.txt`.
    pub fn init(&mut self) {
        let file = File::open("tickers.txt").expect("failed to open tickers.txt");
        let reader = BufReader::new(file);

        self.available_tickers = reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }


    /// Starts the quote generation loop and sends quotes to subscribers.
    pub fn start(&mut self, subscribers: Arc<Mutex<Vec<Sender<StockQuote>>>>) {
        loop {
            if self.available_tickers.is_empty() {
                warn!("no available tickers loaded");
                thread::sleep(Duration::from_millis(1000));
                continue;
            }

            let mut rng = rand::rng();
            let num = rng.random_range(0..self.available_tickers.len());

            if let Some(ticker) = self.available_tickers.get(num) {
                let ticker = ticker.clone();

                if let Some(msg) = self.generate_quote(&ticker) {
                    debug!("msg generated for {}", ticker);
                    let mut subs = subscribers.lock().expect("failed to lock subscribers");
                    debug!("sending quote to {} subscribers", subs.len());

                    subs.retain(|tx| {
                        match tx.send(msg.clone()) {
                            Ok(_) => {
                                debug!("quote generated={}", ticker);
                                true
                            }
                            Err(e) => {
                                warn!("failed to send quote, delete subscriber: {}", e);
                                false
                            }
                        }
                    });
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    }

    /// Generates a random quote for the given ticker.
    pub fn generate_quote(&mut self, ticker: &str) -> Option<StockQuote> {
        let last_price = random::<f64>() * 100.0;

        let is_popular = self
            .popular_tickers
            .iter()
            .any(|t| t == ticker);

        let volume = if is_popular {
            1000 + (random::<f64>() * 5000.0) as u32
        } else {
            100 + (random::<f64>() * 1000.0) as u32
        };

        Some(StockQuote {
            ticker: ticker.to_string(),
            price: last_price,
            volume,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }
}
