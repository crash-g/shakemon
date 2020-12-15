use crate::configuration::ExternalServices;
use crate::errors::CustomError;
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

    let pokemon_description = get_description(pokemon_name, external_services).await?;
    Ok(web::Json(Pokemon {
        name: pokemon_name.to_string(),
        description: pokemon_description,
    }))
}

async fn get_description(
    pokemon_name: &str,
    external_services: web::Data<ExternalServices>,
) -> Result<String, CustomError> {
    let client = Client::default();
    let description =
        pokeapi::get_pokemon_description(pokemon_name, &client, &external_services).await?;
    let translated_description =
        shakespeare::get_translation(&description, &client, &external_services).await?;
    Ok(translated_description)
}
