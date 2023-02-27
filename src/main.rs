use std::net::TcpListener;
use zero2pr::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Unable to bind");
    let port = listener.local_addr().expect("").port().to_string();
    println!("You've got this {port}");
    run(listener)?.await
}
