use actix_web::{get, web, HttpResponse};

use crate::fibonacci::calculate_tribonacci;

#[get("/tribonacci/{n}")]
pub async fn tribonacci(n: web::Path<u32>) -> HttpResponse {
    match calculate_tribonacci(n.into_inner()) {
        Ok(r) => HttpResponse::Ok().json(r),
        e => HttpResponse::BadRequest().json(e),
    }
}
