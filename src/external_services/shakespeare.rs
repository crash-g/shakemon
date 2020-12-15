use crate::errors::FailedRequest;
use actix_web::http::StatusCode;
use reqwest::Client;

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
    base_url: &str,
) -> Result<String, FailedRequest> {
    let url = [base_url, ENDPOINT].concat();
    let body = TextToTranslate {
        text: text.to_string(),
    };
    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(FailedRequest::connection_error)?;

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
