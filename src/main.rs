use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Exchange {
    stocks: HashMap<String, Stock>,
}
impl Exchange {
    fn new() -> Exchange {
        Exchange {
            stocks: HashMap::new(),
        }
    }
    fn add_stock(&mut self, identifier: String, stock: Stock) {
        
        if self.stocks.contains_key(&identifier) {
            panic!("Stock already exists");
        }

        self.stocks.insert(identifier, stock);
    }
}

struct Stock {
    price: f64,
    true_value: f64,
    price_history: Vec<f64>,
}
impl Stock {
    fn fluctuate_price(&mut self, volatility: f64, momentum: f64) {
        let mut rng = thread_rng();
        let price_fluctuation: f64 = rng.gen_range(-1.0..1.0);
        self.price_history.push(self.price);
        self.price += price_fluctuation * volatility - 0.01 * (self.price - self.true_value);
        self.true_value += momentum;
    }
}

fn main() {
    println!("Creating an exchange");
    let mut exchange = Exchange::new();
    exchange.add_stock(
        "AAPL".to_string(),
        Stock {
            price: 100.0,
            true_value: 100.0,
            price_history: Vec::new(),
        },
    );
    println!("stepping through time...");
    for _ in (0..100) {
        exchange
            .stocks
            .get_mut("AAPL")
            .unwrap()
            .fluctuate_price(2.0f64, -0.5);
    }
    exchange
        .stocks
        .get("AAPL")
        .unwrap()
        .price_history
        .iter()
        .for_each(|price| println!("{}", price));
}
