use crate::configuration::ExternalServices;
use crate::errors::ExternalServiceError;
use crate::external_services::{pokeapi, shakespeare};
use actix_web::{client::Client, web, HttpRequest, Result};

#[derive(serde::Serialize)]
pub struct Pokemon {
    name: String,
    description: String,
}

pub async fn get_pokemon_description(
    request: HttpRequest,
    external_services: web::Data<ExternalServices>,
) -> Result<web::Json<Pokemon>> {
    let pokemon_name = request
        .match_info()
        .get("pokemon_name")
        .expect("Failed to find pokemon_name path parameter");

    log::info!("Received description request for {}", pokemon_name);

    let pokeapi_url = &external_services.pokeapi_url;
    let shakespeare_translation_url = &external_services.shakespeare_translation_url;
    let pokemon_description =
        get_description(pokemon_name, pokeapi_url, shakespeare_translation_url).await?;

    Ok(web::Json(Pokemon {
        name: pokemon_name.to_string(),
        description: pokemon_description,
    }))
}

async fn get_description(
    pokemon_name: &str,
    pokeapi_url: &str,
    shakespeare_translation_url: &str,
) -> Result<String, ExternalServiceError> {
    let client = Client::default();

    let description = pokeapi::get_pokemon_description(pokemon_name, &client, pokeapi_url)
        .await
        .map_err(|e| ExternalServiceError::from_pokeapi(e))?;

    let translated_description =
        shakespeare::get_translation(&description, &client, shakespeare_translation_url)
            .await
            .map_err(|e| ExternalServiceError::from_shakespeare_api(e))?;

    Ok(translated_description)
}
