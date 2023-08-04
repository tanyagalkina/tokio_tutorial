use mini_redis::{client, Result};

async fn async_function() {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("async function sleeped enough");
}

// fn main() {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     // `await` is only allowed inside `async` functions and blocks
//     //  async_function().await;
//     rt.block_on(async_function()); // block_on is not async fn
// }

#[tokio::main] // transforms the async fn main() into a synchronous fn main() that initializes a runtime instance and executes the async main function.
async fn main() -> Result<()> {
    // async_function().await; // if I am awaiting here, I am blocking the main thread, so what should I do instead ?
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world!".into()).await?;
    let result = client.get("hello").await?;

    println!(
        "this is what I got back: {:?}",
        String::from_utf8(result.unwrap().to_vec()).unwrap()
    );
    Ok(())
}

// fn main() {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async { async_function().await }); // this is an async block
// }
