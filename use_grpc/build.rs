extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("hello.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}