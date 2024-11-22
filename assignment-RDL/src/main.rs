use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

// Struct to hold Bitcoin pricing data
#[derive(Serialize, Deserialize, Debug)]
struct Bitcoin {
    usd: f64,
}

// Struct to hold Ethereum pricing data
#[derive(Serialize, Deserialize, Debug)]
struct Ethereum {
    usd: f64,
}

// Struct to hold S&P 500 pricing data
#[derive(Serialize, Deserialize, Debug)]
struct SP500 {
    usd: f64,
}

// Trait to standardize fetching and saving pricing data
trait Pricing {
    fn fetch_price(&self, url: &str) -> Option<f64>;
    fn save_to_file(&self, filename: &str, price: f64);
}

// Implementation of Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self, url: &str) -> Option<f64> {
        match ureq::get(url).call() {
            Ok(response) if response.status() == 200 => {
                // Parse JSON response to get Bitcoin price
                let data: serde_json::Value =
                    serde_json::from_reader(response.into_reader()).unwrap_or_else(|err| {
                        eprintln!("Failed to parse JSON: {}", err);
                        serde_json::Value::Null
                    });
                data["bitcoin"]["usd"].as_f64()
            }
            Err(err) => {
                eprintln!("Failed to fetch Bitcoin price: {}", err);
                None
            }
            _ => None,
        }
    }

    fn save_to_file(&self, filename: &str, price: f64) {
        // Append price to a file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open file");
        writeln!(file, "Bitcoin price: ${:.2}", price).expect("Failed to write to file");
    }
}

// Implementation of Pricing trait for Etherum
impl Pricing for Ethereum {
    fn fetch_price(&self, url: &str) -> Option<f64> {
        match ureq::get(url).call() {
            Ok(response) if response.status() == 200 => {
                // Parse JSON response to get Etherum price
                let data: serde_json::Value =
                    serde_json::from_reader(response.into_reader()).unwrap_or_else(|err| {
                        eprintln!("Failed to parse JSON: {}", err);
                        serde_json::Value::Null
                    });
                data["ethereum"]["usd"].as_f64()
            }
            Err(err) => {
                eprintln!("Failed to fetch Ethereum price: {}", err);
                None
            }
            _ => None,
        }
    }

    fn save_to_file(&self, filename: &str, price: f64) {
        // Append price to a file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open file");
        writeln!(file, "Ethereum price: ${:.2}", price).expect("Failed to write to file");
    }
}

// Implementation of Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&self, url: &str) -> Option<f64> {
        match ureq::get(url).call() {
            Ok(response) if response.status() == 200 => {
                // Parse JSON response to get SP500 price
                let data: serde_json::Value =
                    serde_json::from_reader(response.into_reader()).unwrap_or_else(|err| {
                        eprintln!("Failed to parse JSON: {}", err);
                        serde_json::Value::Null
                    });
                data["spdr-s-p-500-etf-trust-defichain"]["usd"].as_f64()
            }
            Err(err) => {
                eprintln!("Failed to fetch S&P 500 price: {}", err);
                None
            }
            _ => None,
        }
    }

    fn save_to_file(&self, filename: &str, price: f64) {
        // Append price to a file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open file");
        writeln!(file, "S&P 500 price: ${:.2}", price).expect("Failed to write to file");
    }
}

fn main() {
    let bitcoin = Bitcoin { usd: 0.0 };
    let ethereum = Ethereum { usd: 0.0 };
    let sp500 = SP500 { usd: 0.0 };

    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,spdr-s-p-500-etf-trust-defichain&vs_currencies=usd";

    loop {
        if let Some(price) = bitcoin.fetch_price(url) {
            bitcoin.save_to_file("bitcoin.txt", price);
        }
        if let Some(price) = ethereum.fetch_price(url) {
            ethereum.save_to_file("ethereum.txt", price);
        }
        if let Some(price) = sp500.fetch_price(url) {
            sp500.save_to_file("sp500.txt", price);
        }

        println!("Data fetched and saved. Waiting for the next interval...");
        thread::sleep(Duration::from_secs(10));
    }
}
