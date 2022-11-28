use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

/// Multiple different commands are multiplexed over a single channel.
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

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // cant clone rx , one receiver or the c in mpsc is 1 consumer
    let (tx, mut rx) = mpsc::channel(32);
    // Clone a `tx` handle for the second f
    // manager is an async handle to a tokio task
    // It accepts request asynchronously and 
    // routes them to the client
    // once the client responds we send the response 


    let manager = tokio::spawn(async move {
        // Open a connection to 127.0.0....
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
    // lets make use of the manager 

    // Spawn two tasks, one setting a value and other querying for key that was
    // set.

    for i in 0..10 {
        println!("i: {}", i);
        let tx_clone = tx.clone();
        let input_string = i.clone().to_string();

        let previous_input_string = (i-1).clone().to_string();

        let t1 = tokio::spawn(async move {
            // make a oneshot channel for the receiver and transmitter rx and tx
            let (resp_tx, resp_rx) = oneshot::channel();
            // make a get command
            let cmd = Command::Get {
                key: previous_input_string,
                resp: resp_tx, // we send resp_tx to the manager so it knows where to send the response
            };

            // Send the GET request
            // this tx is not the same as the one in the manager task
            // but remember the rx this talks to is in the manager task listning in the while loop!
            if tx_clone.send(cmd).await.is_err() {
                eprintln!("connection task shutdown");
                return;
            }

            // Await the response
            let res = resp_rx.await;
            println!("GOT (Get) = {:?}", res);
        });

        let tx_clone = tx.clone();
        let t2 = tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();
            let cmd = Command::Set {
                key: input_string.clone(),
                val: input_string.into(),
                resp: resp_tx,
            };

            // Send the SET request
            if tx_clone.send(cmd).await.is_err() {
                eprintln!("connection task shutdown");
                return;
            }

            // Await the response
            let res = resp_rx.await;
            println!("GOT (Set) = {:?}", res);
        });

        t2.await.unwrap();
        t1.await.unwrap();

    }
    // sleep for 1 second

    manager.await.unwrap();

    
    // Important bit here.. we need to drive all 3 of these futures to completion! manager, t1 and t2 are 0
    // cost abstractions so we need to await to poll them to completion  
}