use std::env;
use std::net::TcpListener;
use tribonacci::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host =
        if env::var("APP_ENVIRONMENT").unwrap_or_else(|_| String::from("local")) == "production" {
            "0.0.0.0"
        } else {
            "127.0.0.1"
        };

    let listener = TcpListener::bind(format!("{}:8000", host))?;
    run(listener)?.await
}
