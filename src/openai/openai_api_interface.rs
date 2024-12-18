use crate::openai::openai_endpoints::OpenAiEndpoints;
use thiserror::Error;
use reqwest;
use serde_json;
use serde;
use log::info;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}


#[derive(serde::Deserialize)]
struct EmbeddingsResponse {
    data: Vec<Data>,
}
#[derive(serde::Deserialize)]
struct Data {
    embedding: Vec<f32>,
}

pub struct OpenAi {
    api_key: String,
    endpoint: Option<String>
}

impl OpenAi {

    pub fn new(api_key: String) -> Self{
        Self { 
            api_key,
            endpoint: None
        }
    }

    pub fn set_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub async fn generate_embeddings(&self, text: &str, model: &str) -> Result<Vec<f32>, ApiError> {

        let client = reqwest::Client::new();
        
        let embedding_endpoint = self.endpoint.as_deref()
            .unwrap_or(OpenAiEndpoints::Embeddings.endpoint());

        let request_body: serde_json::Value = serde_json::json!({
            "input": text,
            "model": model,
            "encoding_format": "float"
        });

        let response = client
            .post(embedding_endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(request_body.to_string())
            .send()
            .await?;

        let json_res: serde_json::Value = match response.json().await{
            Ok(value) => value,
            Err(e) => {
                info!("Embeddings generation failed: {}", e);
                return Err(ApiError::from(e));

            }
        };

        let embedding_response: EmbeddingsResponse = serde_json::from_value(json_res.clone())?;

        let embeddings = &embedding_response.data[0].embedding;

        Ok(embeddings.to_vec())
    }
}