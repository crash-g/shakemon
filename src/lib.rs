//! A server that offers Pokemon descriptions written
//! using Shakespeare's style.

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use lru::LruCache;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod configuration;
mod errors;
mod external_services;
mod routes;
pub mod telemetry;

use configuration::ExternalServices;

/// Initialize the server with the given configuration.
///
/// Three options can be customized:
/// - listener → A TCP listener that the server will listen to
/// - cache_size → The size of the cache that the server uses to store
///   Pokemon descriptions (a value of 0 will deactivate it)
/// - external_services → The addresses of the external services that
///   the server relies to ([PokeAPI](https://pokeapi.co/) for Pokemon
///   descriptions and [funtranslations](https://funtranslations.com/api/)
///   to translate descriptions using Shakespeare's style)
pub fn run(
    listener: TcpListener,
    cache_size: usize,
    external_services: ExternalServices,
) -> std::io::Result<Server> {
    let cache: LruCache<String, String> = LruCache::new(cache_size);
    let thread_safe_cache = Arc::new(Mutex::new(cache));
    let server = HttpServer::new(move || {
        App::new()
            .data(external_services.clone())
            .data(thread_safe_cache.clone())
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
