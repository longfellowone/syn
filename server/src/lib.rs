#![allow(dead_code, unused_variables)]
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Mutex;

pub mod configuration;
mod covid;
mod syn;

pub struct AppData {
    employees: HashMap<String, syn::Employee>,
}

pub fn run(listener: TcpListener) -> Result<Server> {
    let data = AppData {
        employees: syn::employees(),
    };

    let data = web::Data::new(Mutex::new(data));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(data.clone())
            .configure(routes)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/covid", web::get().to(covid::get))
            .service(
                web::scope("/syn")
                    .route("", web::get().to(syn::get))
                    .service(
                        web::resource("/{employee}")
                            .route(web::get().to(syn::find))
                            .route(web::post().to(syn::update)),
                    ),
            ),
    );
}

#[cfg(test)]
pub mod test_util {
    use std::net::TcpListener;

    pub fn run_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = crate::run(listener).expect("failed to start server");

        actix_rt::spawn(server);

        format!("http://127.0.0.1:{}", port)
    }
}
