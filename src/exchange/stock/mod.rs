use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

pub struct Stock  {
    pub ticker: String,
    pub price: f64,
    pub true_value: f64,
    pub price_history: Vec<f64>,
}
impl Stock {


    pub fn fluctuate_price(price: &f64, true_value: &f64, volatility: &f64, momentum: &f64) -> (f64,f64) {

        let mut rng = thread_rng();
        let mut new_price = price + (rng.gen_range(-1.0..1.0) * volatility * true_value);
        
        if new_price < 0.0 {
            new_price = 0.0;
        }

        let mut new_true_value = true_value + (rng.gen_range(-1.0..1.0) * momentum * true_value);
        if new_true_value < 0.0 {
            new_true_value = 0.0;
        }
        println!("New price: {}, new true value: {}", new_price, new_true_value);

        (new_price, new_true_value)
    }

    pub fn start_stock_fluctuation(&mut self) {

        thread::spawn(move || {
            loop {
                fluctuate_price(&self.price, &self.true_value, &0.1, &0.1);
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

}
