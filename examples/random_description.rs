use rand::Rng;
use reqwest::{Client, StatusCode};

const POKEAPI_URL: &str = "https://pokeapi.co/api/v2/pokemon";
const SHAKEMON_URL: &str = "http://localhost:8000/pokemon";

#[derive(serde::Deserialize, Debug)]
struct PokemonList {
    next: Option<String>,
    results: Vec<PokemonName>,
}

#[derive(serde::Deserialize, Debug)]
struct PokemonName {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct PokemonDescription {
    pub name: String,
    pub description: String,
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let mut next_url = Some(POKEAPI_URL.to_string());

    while next_url.is_some() {
        next_url = look_for_pokemons(&client, &next_url.unwrap()).await;
    }
}

async fn look_for_pokemons(client: &Client, url: &str) -> Option<String> {
    let response = client.get(url).send().await.unwrap();

    if response.status() == StatusCode::OK {
        let pokemon_list: PokemonList = response.json().await.unwrap();

        let num_pokemons = pokemon_list.results.len();

        let mut num = rand::thread_rng().gen_range(0, num_pokemons + 1);

        while num < num_pokemons {
            get_pokemon_description(client, &pokemon_list.results[num].name).await;
            num = rand::thread_rng().gen_range(0, num_pokemons + 1);
        }

        pokemon_list.next
    } else {
        println!("Got status code {}", response.status());
        None
    }
}

async fn get_pokemon_description(client: &Client, pokemon_name: &str) {
    let response = client
        .get(&format!("{}/{}", SHAKEMON_URL, pokemon_name))
        .send()
        .await
        .unwrap();

    if response.status() == StatusCode::OK {
        let pokemon_description: PokemonDescription = response.json().await.unwrap();

        assert_eq!(pokemon_name, pokemon_description.name);
        println!("{}: {}", pokemon_name, pokemon_description.description);
    } else {
        println!("Got status code {}", response.status());
    }
}
