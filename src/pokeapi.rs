use crate::configuration::ExternalServices;
use actix_web::client::Client;

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
) -> String {
    let url = [&external_services.pokeapi_url, ENDPOINT, "/", pokemon_name].concat();
    let res = client.get(url).send().await;
    // TODO do not call unwrap
    let r: Pokemon = res.unwrap().json().await.unwrap();
    // TODO choose better what to return:
    r.flavor_text_entries[0].flavor_text.to_string()
}
