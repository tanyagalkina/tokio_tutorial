use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, conn) = listener.accept().await.unwrap();
        println!("Accepted connection from {:?}", conn);
        // tokio::spawn(async move { // this makes it possible not to block the main thread
            process(socket).await;
        // });
    }
}

async fn process(socket: TcpStream) {
    // converts byte streams into redis frames
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        println!("I am thinking ...");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        println!("I am done thinking");
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
