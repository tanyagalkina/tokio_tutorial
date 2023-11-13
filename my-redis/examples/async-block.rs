fn function() -> i8 {
    println!("async function sleeped enough");
    42
}

async fn async_function() -> i8 {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("async function sleeped enough");
    42
}

#[tokio::main]
async fn main() {

    let async_block = async {
        42
    };
    // can interact with the spawned task using JoinHandle ( the type of handle )
    let handle = tokio::spawn(async { function() });
    assert!(handle.await.unwrap() == 42); // `Result<i8, JoinError>` is returned by `JoinHandle::await`
                                          // The error returned is comping from the task( either panics or gets forcefully cancelled by the runtime shutting down ), and not from the called function itself

    let handle_2 = tokio::spawn(async { async_function().await });
    assert!(handle_2.await.unwrap() == 42);

    let handle_3 = tokio::spawn(async { async_function().await });
    assert!(handle_3.await.unwrap() == 42);
    let handle_4 = tokio::spawn(async_block);
    assert!(handle_4.await.unwrap() == 42);
}
