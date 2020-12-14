use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub mod configuration;
mod errors;
mod external_services;
mod routes;
pub mod telemetry;

use configuration::ExternalServices;

pub fn run(listener: TcpListener, external_services: ExternalServices) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(external_services.clone())
            .route("/health_check", web::get().to(routes::health_check))
            .route(
                "/pokemon/{pokemon_name}",
                web::get().to(routes::get_pokemon_description),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
