use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind");
    let port = listener.local_addr().expect("").port().to_string();
    println!("You've got this {port}");
    zero2pr::run(listener)?.await
}
