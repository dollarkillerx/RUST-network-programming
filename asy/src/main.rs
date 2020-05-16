use futures::executor::block_on;

async fn study_song() {
    println!("Study Song");
}

async fn sing() {
    println!("Sing");
}

async fn performance_singing() {
    study_song().await;
    sing().await;
}

async fn dancing() {
    println!("Dancing");
}

async fn awkward() {
    println!("Awkward");
    let s1 = performance_singing();
    let s2 = dancing();

    futures::join!(s1,s2);
}

fn main() {
    block_on(awkward());
    println!("Hello, world!");
}
