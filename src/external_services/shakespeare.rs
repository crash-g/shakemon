use crate::configuration::ExternalServices;
use crate::errors::FailedRequest;
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
) -> Result<String, FailedRequest> {
    let url = [&external_services.shakespeare_translation_url, ENDPOINT].concat();
    let body = TextToTranslate {
        text: text.to_string(),
    };
    let mut response = client
        .post(url)
        .send_json(&body)
        .await
        .map_err(|e| FailedRequest::connection_error(e))?;

    if response.status() == StatusCode::OK {
        let translation: Translation = response
            .json()
            .await
            .map_err(|e| FailedRequest::invalid_payload(text.to_string(), e))?;

        Ok(translation.contents.translated)
    } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
        Err(FailedRequest::too_many_requests())
    } else {
        Err(FailedRequest::unexpected_status_code(
            text.to_string(),
            response.status(),
        ))
    }
}
