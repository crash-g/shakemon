use crate::errors::FailedRequest;
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
    base_url: &str,
) -> Result<String, FailedRequest> {
    let url = [base_url, ENDPOINT, "/", pokemon_name].concat();
    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|e| FailedRequest::connection_error(e))?;

    if response.status() == StatusCode::OK {
        let pokemon: Pokemon = response
            .json()
            .await
            .map_err(|e| FailedRequest::invalid_payload(pokemon_name.to_string(), e))?;
        // TODO choose better what to return:
        Ok(pokemon.flavor_text_entries[0].flavor_text.to_string())
    } else if response.status() == StatusCode::NOT_FOUND {
        Err(FailedRequest::not_found(pokemon_name.to_string()))
    } else {
        Err(FailedRequest::unexpected_status_code(
            pokemon_name.to_string(),
            response.status(),
        ))
    }
}
