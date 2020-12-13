mod common;

#[actix_rt::test]
async fn get_pokemon_description_works() {
    let address = common::spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/pokemon/test_name", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let pokemon: common::Pokemon = response.json().await.unwrap();
    assert_eq!("test_name".to_string(), pokemon.name);
}
