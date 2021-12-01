use actix_web::{HttpResponse, Responder};
use chrono::Utc;
use chrono_tz::Canada;

pub async fn health_check() -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] Syn punch in/out", time);

    HttpResponse::Ok()
}

pub async fn punchin() -> impl Responder {
    HttpResponse::Ok().body("punched in")
}

pub async fn punchout() -> impl Responder {
    HttpResponse::Ok().body("punched out")
}

#[cfg(test)]
mod test {
    use crate::test_util;

    #[actix_rt::test]
    async fn test_v1_syn_get_is_success() {
        let address = test_util::run_app();
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/v1/syn", address))
            .send()
            .await
            .expect("failed to execute request to /v1/syn");

        assert!(response.status().is_success())
    }
}
