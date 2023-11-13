use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, conn) = listener.accept().await.unwrap();
        println!("Accepted connection from {:?}", conn);
        let db = db.clone();
        tokio::spawn(async move {
            // this makes it possible not to block the main thread
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};


    let mut connection = Connection::new(socket);


    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string()) // what is this doing ?
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null // what is this doing ?
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }

    // Old version
    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);
    //     println!("I am thinking ...");
    //     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    //     println!("I am done thinking");
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }
}
