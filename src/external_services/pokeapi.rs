use crate::configuration::ExternalServices;
use crate::errors::CustomError;
use actix_web::client::Client;
use actix_web::http::StatusCode;

const ENDPOINT: &str = "/api/v2/pokemon-species";

#[derive(serde::Deserialize)]
struct Pokemon {
    flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(serde::Deserialize)]
struct FlavorTextEntry {
    flavor_text: String,
    language: Language,
}

#[derive(serde::Deserialize)]
struct Language {
    name: String,
}

pub(crate) async fn get_pokemon_description(
    pokemon_name: &str,
    client: &Client,
    external_services: &ExternalServices,
) -> Result<String, CustomError> {
    let url = [&external_services.pokeapi_url, ENDPOINT, "/", pokemon_name].concat();
    let mut response = client.get(url).send().await.map_err(|e| {
        log::error!(
            "An error occurred while querying the pokeapi endpoint. Error: {:?}",
            e
        );
        CustomError::InternalServerError
    })?;

    if response.status() == StatusCode::OK {
        let pokemon: Pokemon = response.json().await.map_err(|e| {
            log::error!(
                "An error occurred while deserializing the JSON payload \
                 from the pokeapi endpoint. Error: {:?}",
                e
            );
            CustomError::InternalServerError
        })?;
        // TODO choose better what to return:
        Ok(pokemon.flavor_text_entries[0].flavor_text.to_string())
    } else if response.status() == StatusCode::NOT_FOUND {
        log::debug!("Pokemon {} has not been found", pokemon_name);
        Err(CustomError::NotFound)
    } else {
        log::error!(
            "The pokeapi endpoint returned an unexpected status code: {}. \
             Expected either 200 or 404.",
            response.status()
        );
        Err(CustomError::InternalServerError)
    }
}
