use actix_web::{web, HttpRequest, Result};

#[derive(serde::Serialize)]
pub struct Pokemon {
    name: String,
    description: String,
}

pub async fn get_pokemon_description(req: HttpRequest) -> Result<web::Json<Pokemon>> {
    let pokemon_name = req
        .match_info()
        .get("pokemon_name")
        .expect("Failed to find pokemon_name path parameter");

    let pokemon_description = get_description(pokemon_name);

    Ok(web::Json(Pokemon {
        name: pokemon_name.to_string(),
        description: pokemon_description,
    }))
}

fn get_description(pokemon_name: &str) -> String {
    "TODO".to_string()
}
