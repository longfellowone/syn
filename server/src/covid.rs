use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use chrono_tz::Canada;

pub async fn get(req: HttpRequest) -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("{} -  Covid sign in", time);

    HttpResponse::Ok()
}
