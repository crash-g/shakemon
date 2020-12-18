use crate::configuration::ExternalServices;
use crate::errors::ExternalServiceError;
use crate::external_services::{pokeapi, shakespeare};
use actix_web::{web, HttpRequest, Result};
use lru::LruCache;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Serialize)]
pub struct Pokemon {
    name: String,
    description: String,
}

pub async fn get_pokemon_description(
    request: HttpRequest,
    external_services: web::Data<ExternalServices>,
    cache: web::Data<Arc<Mutex<LruCache<String, String>>>>,
) -> Result<web::Json<Pokemon>> {
    let pokemon_name = request
        .match_info()
        .get("pokemon_name")
        .expect("Failed to find pokemon_name path parameter")
        .to_string();

    log::debug!("Received description request for {}", pokemon_name);

    let cached_description = cache.lock().await.get(&pokemon_name).cloned();
    let pokemon_description = match cached_description {
        Some(description) => description,
        None => {
            let description = get_description(&pokemon_name, &external_services).await?;
            cache
                .lock()
                .await
                .put(pokemon_name.clone(), description.clone());
            description
        }
    };

    log::debug!(
        "{}: the description is {}",
        pokemon_name,
        pokemon_description
    );

    Ok(web::Json(Pokemon {
        name: pokemon_name,
        description: pokemon_description,
    }))
}

async fn get_description(
    pokemon_name: &str,
    external_services: &ExternalServices,
) -> Result<String, ExternalServiceError> {
    let client = Client::new();

    let pokeapi_url = &external_services.pokeapi_url;
    let shakespeare_translation_url = &external_services.shakespeare_translation_url;

    let description = pokeapi::get_pokemon_description(pokemon_name, &client, pokeapi_url)
        .await
        .map_err(ExternalServiceError::from_pokeapi)?;

    let translated_description =
        shakespeare::get_translation(&description, &client, shakespeare_translation_url)
            .await
            .map_err(ExternalServiceError::from_shakespeare_api)?;

    Ok(translated_description)
}
