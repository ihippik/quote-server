use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use crate::server::stock::StockQuote;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{random, Rng};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader};
use log::info;

pub struct QuoteGenerator {
    popular_tickers: Vec<String>,
    available_tickers: Vec<String>,
}

impl QuoteGenerator {
    pub fn new(tickers: Vec<String>) -> Self {
        let all = vec![];
        Self { popular_tickers: tickers, available_tickers: all }
    }

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

    pub fn start(&mut self, subscribers: Arc<Mutex<Vec<Sender<StockQuote>>>>) {
        loop {
            for tx in subscribers.lock().unwrap().iter() {
                let num = rand::rng().random_range(0..self.available_tickers.len());

                if let Some(ticker) = self.available_tickers.get(num) {
                    let ticker = ticker.clone(); // делаем String

                    if let Some(msg) = self.generate_quote(ticker.as_ref()) {
                        tx.send(msg).unwrap();
                        info!("quote generated={}",ticker)
                    }
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    }

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
