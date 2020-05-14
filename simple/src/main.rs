use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize)]
struct Point {
    name: String,
    age: u8,
}

fn main() {
    let a = Point{
        name: String::from("Aaa"),
        age: 18,
    };
    let c = serde_json::to_string(&a).unwrap();
    println!("c: {}",c);

    let cs:Point = serde_json::from_str(c.as_str()).unwrap();
    println!("c: {:?}",cs);
}
