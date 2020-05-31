use server::server::Server;

fn main() -> std::io::Result<()> {
    let ser = Server::new("127.0.0.1:8081".to_string());

    ser.run(10)?;

    Ok(())
}