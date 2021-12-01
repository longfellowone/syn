use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use chrono_tz::Canada;

pub async fn health_check(req: HttpRequest) -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] Covid sign in", time);

    HttpResponse::Ok()
}

#[cfg(test)]
mod test {
    use crate::test_util;

    #[actix_rt::test]
    async fn test_v1_covid_get_is_success() {
        let address = test_util::run_app();
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/v1/covid", address))
            .send()
            .await
            .expect("failed to execute request to /v1/covid");

        assert!(response.status().is_success())
    }
}
