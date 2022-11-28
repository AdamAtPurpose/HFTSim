use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));


    loop {
        let mutex_ref = db.clone();
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        // a new tokio task is spawned for each inbound socket,
        // note for Adam: move keyword transfers ownership of that socket to the new task (green thread)
        let handle = tokio::spawn(async move {
            let the_future = process(socket, mutex_ref);
            the_future.await;
        });
        let out = handle.await.unwrap();
        println!("::: out = {:?}", out);
    }
}

async fn process(socket: TcpStream, mutex_ref: Arc<Mutex<HashMap<String, Bytes>>>) {

    // A hashmap is used to store data
    // let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = mutex_ref.lock().unwrap();
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = mutex_ref.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
