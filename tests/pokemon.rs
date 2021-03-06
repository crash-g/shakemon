use reqwest::{Client, StatusCode};
use wiremock::matchers::{method, path};
use wiremock::{Mock, Request, ResponseTemplate};

mod data;
mod utils;

use data::{pokeapi, shakespeare};

#[actix_rt::test]
async fn get_pokemon_description_works() {
    let pokemon_name_ref = "charizard";
    let description_ref = "Description from pokeapi";
    let translated_description_ref = "Translated description from shakespeare";

    let (address, mock_pokeapi_server, mock_shakespeare_server) =
        utils::spawn_app_with_mocked_external_services().await;

    let flavor_text_entry = pokeapi::FlavorTextEntry {
        flavor_text: description_ref.to_string(),
        language: pokeapi::Language {
            name: "en".to_string(),
        },
    };
    let pokemon = pokeapi::Pokemon {
        flavor_text_entries: vec![flavor_text_entry],
    };
    Mock::given(method("GET"))
        .and(path(format!("{}/{}", pokeapi::ENDPOINT, pokemon_name_ref)))
        .respond_with(ResponseTemplate::new(200).set_body_json(pokemon))
        .expect(1)
        .mount(&mock_pokeapi_server)
        .await;

    let description = shakespeare::Translation {
        contents: shakespeare::TranslationContents {
            translated: translated_description_ref.to_string(),
        },
    };
    Mock::given(method("POST"))
        .and(path(shakespeare::ENDPOINT.to_string()))
        .respond_with(ResponseTemplate::new(200).set_body_json(description))
        .expect(1)
        .mount(&mock_shakespeare_server)
        .await;

    let client = Client::new();

    let response = client
        .get(&format!("{}/pokemon/{}", address, pokemon_name_ref))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let pokemon: data::Pokemon = response.json().await.unwrap();
    assert_eq!(pokemon_name_ref, &pokemon.name);
    assert_eq!(translated_description_ref, &pokemon.description);

    // We repeat the test once to make sure that the cache kicks in
    // and we get the same answer without querying external services.
    let response = client
        .get(&format!("{}/pokemon/{}", address, pokemon_name_ref))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let pokemon: data::Pokemon = response.json().await.unwrap();
    assert_eq!(pokemon_name_ref, &pokemon.name);
    assert_eq!(translated_description_ref, &pokemon.description);
}

#[actix_rt::test]
async fn get_pokemon_description_not_found() {
    let pokemon_name_ref = "charizard";

    let (address, mock_pokeapi_server, mock_shakespeare_server) =
        utils::spawn_app_with_mocked_external_services().await;

    Mock::given(method("GET"))
        .and(path(format!("{}/{}", pokeapi::ENDPOINT, pokemon_name_ref)))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&mock_pokeapi_server)
        .await;

    Mock::given(move |_request: &Request| true)
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&mock_shakespeare_server)
        .await;

    let client = Client::new();

    let response = client
        .get(&format!("{}/pokemon/{}", address, pokemon_name_ref))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn get_pokemon_description_too_many_requests() {
    let pokemon_name_ref = "charizard";
    let description_ref = "Description from pokeapi";

    let (address, mock_pokeapi_server, mock_shakespeare_server) =
        utils::spawn_app_with_mocked_external_services().await;

    let flavor_text_entry = pokeapi::FlavorTextEntry {
        flavor_text: description_ref.to_string(),
        language: pokeapi::Language {
            name: "en".to_string(),
        },
    };
    let pokemon = pokeapi::Pokemon {
        flavor_text_entries: vec![flavor_text_entry],
    };
    Mock::given(method("GET"))
        .and(path(format!("{}/{}", pokeapi::ENDPOINT, pokemon_name_ref)))
        .respond_with(ResponseTemplate::new(200).set_body_json(pokemon))
        .expect(1)
        .mount(&mock_pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .and(path(shakespeare::ENDPOINT.to_string()))
        .respond_with(ResponseTemplate::new(429))
        .expect(1)
        .mount(&mock_shakespeare_server)
        .await;

    let client = Client::new();

    let response = client
        .get(&format!("{}/pokemon/{}", address, pokemon_name_ref))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
}
