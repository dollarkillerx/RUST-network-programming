use futures::executor;
async fn hello() {
    println!("Hello Rust Async")
}

pub fn test1() {
    let f1 = hello();
    executor::block_on(f1);
}

