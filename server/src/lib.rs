#![allow(dead_code, unused_variables)]
use crate::configuration::Configuration;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Mutex;

mod configuration;
mod covid;
mod syn;

pub struct AppData {
    employees: HashMap<String, syn::Employee>,
}

pub async fn run() -> Result<()> {
    let config = Configuration::new()?;

    let data = AppData {
        employees: syn::employees(),
    };

    let data = web::Data::new(Mutex::new(data));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(data.clone())
            .configure(routes)
    })
    .bind(config.server.addr())?
    .run()
    .await?;

    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::resource("/covid").route(web::get().to(covid::get)))
            .service(
                web::scope("/syn")
                    .service(web::resource("").route(web::get().to(syn::get)))
                    .service(web::resource("/{employee}").route(web::get().to(syn::find)))
                    .service(web::resource("/{employee}").route(web::put().to(syn::put)))
                    .service(web::resource("/punchin").route(web::post().to(syn::punchin)))
                    .service(web::resource("/punchout").route(web::post().to(syn::punchout))),
            ),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    async fn start_app() -> Result<()> {
        run().await
    }

    #[actix_rt::test]
    async fn test_something() {
        start_app().await;
    }
}