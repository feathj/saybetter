use base64::decode;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;

use std::path::PathBuf;
use tempfile::NamedTempFile;

const GOOGLE_TTS_API_URL: &str = "https://texttospeech.googleapis.com/v1/text:synthesize";

#[derive(Serialize)]
struct InputText {
    text: String,
}

#[derive(Serialize)]
struct VoiceSelectionParams {
    language_code: String,
    name: String,
}

#[derive(Serialize)]
struct AudioConfig {
    audio_encoding: String,
}

#[derive(Serialize)]
struct TextToSpeechRequest {
    input: InputText,
    voice: VoiceSelectionParams,
    audio_config: AudioConfig,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    NormalResponse {
        #[serde(rename = "audioContent")]
        audio_content: String,
    },
    ErrorResponse {
        error: ApiError,
    },
}

pub async fn runtts(
    token: &str,
    project_name: &str,
    language: &str,
    voice: &str,
    message: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert(
        "x-goog-user-project",
        HeaderValue::from_str(&format!("{}", project_name))?,
    );

    let request_body = TextToSpeechRequest {
        input: InputText {
            text: message.to_string(),
        },
        voice: VoiceSelectionParams {
            language_code: language.to_string(),
            name: voice.to_string(),
        },
        audio_config: AudioConfig {
            audio_encoding: "LINEAR16".to_string(),
        },
    };

    let mut temp_file = NamedTempFile::new()?;

    let request_body_json = serde_json::to_string(&request_body)?;
    {
        let res = client
            .post(GOOGLE_TTS_API_URL)
            .headers(headers)
            .body(request_body_json)
            .send()
            .await?;

        let api_response = res.json::<ApiResponse>().await?;
        let audio_content = match api_response {
            ApiResponse::NormalResponse { audio_content } => audio_content,
            ApiResponse::ErrorResponse { error } => {
                return Err(error.message.into());
            }
        };

        let binary_data = decode(audio_content)?;
        temp_file.write_all(&binary_data)?;
    }

    return Ok(temp_file.into_temp_path().keep()?);
}
