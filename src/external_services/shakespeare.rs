use crate::configuration::ExternalServices;
use actix_web::client::Client;

const ENDPOINT: &str = "/translate/shakespeare";

#[derive(serde::Serialize)]
struct TextToTranslate {
    text: String,
}

#[derive(serde::Deserialize)]
struct Translation {
    contents: TranslationContents,
}

#[derive(serde::Deserialize)]
struct TranslationContents {
    translated: String,
}

pub(crate) async fn get_translation(
    text: &str,
    client: &Client,
    external_services: &ExternalServices,
) -> String {
    let url = [&external_services.shakespeare_translation_url, ENDPOINT].concat();
    let body = TextToTranslate {
        text: text.to_string(),
    };
    let res = client.post(url).send_json(&body).await;
    // TODO do not call unwrap
    let translation: Translation = res.unwrap().json().await.unwrap();
    translation.contents.translated
}
