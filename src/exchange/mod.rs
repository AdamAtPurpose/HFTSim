use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub mod stock;
use stock::Stock;
pub struct Exchange {
    pub stocks: HashMap<String, Stock>,
}
impl Exchange {

    pub fn new() -> Exchange {
        let mut new_exchange: Exchange = Exchange {
            stocks: HashMap::new(),
        };

        println!("New exchange created!");
        new_exchange.add_stock(
            "AAPL".to_string(),
            Stock{
                ticker: "AAPL".to_string(),
                price: 100.0,
                true_value: 100.0,
                price_history: Vec::new(),
            }
        );
        new_exchange.add_stock(
            "GOOG".to_string(),
            Stock{
                ticker: "GOOG".to_string(),
                price: 100.0,
                true_value: 100.0,
                price_history: Vec::new(),
            }
        );
        new_exchange
    }
    
    fn add_stock(&mut self, identifier: String, stock: Stock) {
        
        if self.stocks.contains_key(&identifier) {
            panic!("Stock already exists");
        }
        println!("Adding stock {} to exchange", identifier);
        self.stocks.insert(identifier, stock);
    }
}