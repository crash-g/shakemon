use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub mod configuration;
mod routes;
pub mod telemetry;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
