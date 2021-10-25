use std::net::TcpListener;
use tribonacci::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    run(listener)?.await
}
