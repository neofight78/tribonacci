use actix_web::dev::Server;
use actix_web::error::PathError;
use actix_web::error::PathError::Deserialize;
use actix_web::web::PathConfig;
use actix_web::{error, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

use crate::routes::tribonacci;

/// Converts path errors to 400 status with the error formatted as json
fn path_error_handler(err: PathError, _: &HttpRequest) -> actix_web::Error {
    let mut message = String::from("invalid path");

    if let Deserialize(ref e) = err {
        message = e.to_string();
    }

    let body = Err::<String, String>(message);

    error::InternalError::from_response(err, HttpResponse::BadRequest().json(body)).into()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(PathConfig::default().error_handler(path_error_handler))
            .service(tribonacci)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
