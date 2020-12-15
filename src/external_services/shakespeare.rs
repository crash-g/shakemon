use crate::configuration::ExternalServices;
use crate::errors::CustomError;
use actix_web::client::Client;
use actix_web::http::StatusCode;

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
) -> Result<String, CustomError> {
    let url = [&external_services.shakespeare_translation_url, ENDPOINT].concat();
    let body = TextToTranslate {
        text: text.to_string(),
    };
    let mut response = client.post(url).send_json(&body).await.map_err(|e| {
        log::error!(
            "An error occurred while querying the shakespeare translation endpoint. \
             Error: {:?}",
            e
        );
        CustomError::InternalServerError
    })?;

    if response.status() == StatusCode::OK {
        let translation: Translation = response.json().await.map_err(|e| {
            log::error!(
                "An error occurred while deserializing the JSON payload \
                 from the shakespeare translation endpoint. Error: {:?}",
                e
            );
            CustomError::InternalServerError
        })?;

        Ok(translation.contents.translated)
    } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
        log::debug!("The shakespeare translation endpoint returned TOO_MANY_REQUESTS");
        Err(CustomError::TooManyRequests)
    } else {
        log::error!(
            "The shakespeare translation endpoint returned an unexpected status code: {}. \
             Expected either 200 or 429.",
            response.status()
        );
        Err(CustomError::InternalServerError)
    }
}
