use crate::errors::FailedRequest;
use reqwest::{Client, StatusCode};

const ENDPOINT: &str = "/api/v2/pokemon-species";
const LANGUAGE: &str = "en";

#[derive(serde::Deserialize, Debug)]
struct Pokemon {
    flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(serde::Deserialize, Debug)]
struct FlavorTextEntry {
    flavor_text: String,
    language: Language,
}

#[derive(serde::Deserialize, Debug)]
struct Language {
    name: String,
}

pub(crate) async fn get_pokemon_description(
    pokemon_name: &str,
    client: &Client,
    base_url: &str,
) -> Result<String, FailedRequest> {
    let url = [base_url, ENDPOINT, "/", pokemon_name].concat();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(FailedRequest::connection_error)?;

    if response.status() == StatusCode::OK {
        let pokemon: Pokemon = response
            .json()
            .await
            .map_err(|e| FailedRequest::invalid_payload(pokemon_name.to_string(), e))?;

        extract_description(pokemon)
            .ok_or_else(|| FailedRequest::not_found(pokemon_name.to_string()))
    } else if response.status() == StatusCode::NOT_FOUND {
        Err(FailedRequest::not_found(pokemon_name.to_string()))
    } else {
        Err(FailedRequest::unexpected_status_code(
            pokemon_name.to_string(),
            response.status(),
        ))
    }
}

fn extract_description(pokemon: Pokemon) -> Option<String> {
    pokemon
        .flavor_text_entries
        .into_iter()
        .filter(|entry| entry.language.name == LANGUAGE)
        .max_by_key(|entry| entry.flavor_text.len())
        .map(|entry| entry.flavor_text)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_the_longest_english_description_is_picked() {
        let short_description = "ab";
        let longest_english_description = "abcd";
        let longest_italian_description = "abcde";

        let english = "en";
        let italian = "it";

        let flavor_text_entries = vec![
            FlavorTextEntry {
                flavor_text: longest_italian_description.to_string(),
                language: Language {
                    name: italian.to_string(),
                },
            },
            FlavorTextEntry {
                flavor_text: short_description.to_string(),
                language: Language {
                    name: english.to_string(),
                },
            },
            FlavorTextEntry {
                flavor_text: longest_english_description.to_string(),
                language: Language {
                    name: english.to_string(),
                },
            },
            FlavorTextEntry {
                flavor_text: short_description.to_string(),
                language: Language {
                    name: english.to_string(),
                },
            },
        ];

        let pokemon = Pokemon {
            flavor_text_entries,
        };

        let description = extract_description(pokemon);
        assert!(description.is_some());
        assert_eq!(longest_english_description, description.unwrap());
    }
}
