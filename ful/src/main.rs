fn main() {
    println!("Hello, world!");
}

trait SimpleFutures {
    type Output;
    fn poll(&mut self,wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

