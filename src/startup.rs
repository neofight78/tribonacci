use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

use crate::routes::tribonacci;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(tribonacci))
        .listen(listener)?
        .run();

    Ok(server)
}
