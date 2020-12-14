use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
mod data;

use data::pokeapi;

#[actix_rt::test]
async fn get_pokemon_description_works() {
    let pokemon_name = "charizard";

    let mock_pokeapi_server = MockServer::start().await;
    let mock_shakespeare_server = MockServer::start().await;
    let address = common::spawn_app_with_mocked_external_services(
        &mock_pokeapi_server,
        &mock_shakespeare_server,
    );

    let client = reqwest::Client::new();

    let flavor_text_entry = pokeapi::FlavorTextEntry {
        flavor_text: "Text got from pokeapi".to_string(),
        language: pokeapi::Language {
            name: "en".to_string(),
        },
    };
    let pokemon = pokeapi::Pokemon {
        flavor_text_entries: vec![flavor_text_entry],
    };
    Mock::given(method("GET"))
        .and(path(format!("{}/{}", pokeapi::ENDPOINT, pokemon_name)))
        .respond_with(ResponseTemplate::new(200).set_body_json(pokemon))
        .mount(&mock_pokeapi_server)
        .await;

    let response = client
        .get(&format!("{}/pokemon/{}", address, pokemon_name))
        .send()
        .await
        .expect("Failed to execute request.");

    println!("status: {:?}", response.status());
    assert!(response.status().is_success());
    let pokemon: data::Pokemon = response.json().await.unwrap();
    assert_eq!(pokemon_name, &pokemon.name);
}
