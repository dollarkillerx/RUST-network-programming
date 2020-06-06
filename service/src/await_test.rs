use futures::{self,executor};
use std::{thread,time};

async fn sing() {
    let stu = study_song();
    stu.await;

    println!("Sing");
}

async fn study_song() {
    thread::sleep(time::Duration::from_secs(3 ));
    println!("Study Song")
}

async fn dancing() {
    println!("dancing")
}

async fn performance() {
    let sing = sing();
    let dancing = dancing();

    futures::join!(sing,dancing);
}

pub fn await_test() {
    executor::block_on(performance());
}