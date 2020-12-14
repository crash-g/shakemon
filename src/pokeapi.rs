use crate::configuration::ExternalServices;
use actix_web::client::Client;

const ENDPOINT: &str = "/api/v2/pokemon-species";

#[derive(serde::Deserialize)]
pub(crate) struct Pokemon {
    pub flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(serde::Deserialize)]
pub(crate) struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
}

#[derive(serde::Deserialize)]
pub(crate) struct Language {
    pub name: String,
}

pub(crate) async fn get_pokemon_description(
    pokemon_name: &str,
    client: &Client,
    external_services: &ExternalServices,
) -> String {
    let url = [&external_services.pokeapi_url, ENDPOINT, "/", pokemon_name].concat();
    let res = client
        .get(url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    // TODO do not call unwrap
    let r: Pokemon = res.unwrap().json().await.unwrap();
    // TODO choose better what to return:
    r.flavor_text_entries[0].flavor_text.to_string()
}
