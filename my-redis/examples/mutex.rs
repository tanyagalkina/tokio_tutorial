use std::sync::Mutex;
use std::rc::Rc;
use std::sync::Arc;

// static lifetime example:

static STATIC_NUMBER: i16 = 42;

// 2. static  as Trait bound  means that the type does not contain any non-static references


fn main() {
    let m = Mutex::new(5);

    {
        // LockResult<T> = MutexGuard<'a, T>
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // MutexGuard<'a, T> is dropped here ( because is going out of scope )
    println!("Hello, Mutex!");
    println!("m = {:?}", m);

    // ***************

    // let counter = Rc::new(Mutex::new(0)); Cannot be sent between threads safely
    // the Trait `Send `is not implemented for `Rc<Mutex<i32>>`
    let counter = Arc::new(Mutex::new(0)); // Atomic Reference Counting
    let mut handles = vec![];

    for _ in 0..10 {
        // MutexGuard<'a, T>
        let counter = Arc::clone(&counter); // we are cloning the Arc smart pointer
        // if we don't clone the Arc smart pointer, we will get an error:
        // value moved into closure here, in previous iteration of loop
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
