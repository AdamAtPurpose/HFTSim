use std::sync::Arc;

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {

    println!("Starting client");
    let (tx, mut rx) = mpsc::channel(32);
    
    let manager = tokio::spawn(async move {
        // Open a connection to 127.0.0....

        println!("Inside thread");
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        // await the rx channel i.e drive that receiver to completion
        while let Some(cmd) = rx.recv().await {
            // theres two commands get and set which we made Enums for
            // we match the command then call the appropriate mini redis command whether thats get or set
            // then we await the response one shot channel
            // then when we get a response we send that back to to responder through resp which is a oneshot channel
            // resp is sent with the get request, it's part of a GET command, so the server needs to meake a oneshot channel and send it in the request!
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });

    println!("Manager spawned");

    // send set commands on 10 second intervals
    println!("beginning to send new set commands to server on a 10s interval");
    let stocks = vec!["TSLA", "AAPL", "MSFT", "AMZN", "GOOG", "FB", "NFLX", "NVDA", "PYPL", "ADBE"];
    let mut prices = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];

    loop {

        // flux the prices
        for (stock, price) in stocks.iter().zip(prices.iter()) {

            // mutate the price
            let price_to_send = price + rand::random::<i32>() % 10;

            // TODO: multi-thread this  
            let (resp_tx, resp_rx) = oneshot::channel();
            let cmd = Command::Set {
                key: stock.to_string(),
                val: price_to_send.to_string().into(),
                resp: resp_tx,
            };
            tx.send(cmd).await.unwrap();

            let res = resp_rx.await.unwrap();
            println!("set {} to {}: {:?}", stock, price_to_send, res);
        }

        // sleep for 10 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }


}