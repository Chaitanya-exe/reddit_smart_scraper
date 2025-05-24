use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest <'a>{
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
pub struct EmbeddingResponse{
    pub embedding: Vec<f32>
}

pub async fn get_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>>{
    let client = Client::new();

    let res = client.post("http://localhost:11434/api/embeddings")
        .json(&EmbeddingRequest{
            model: "nomic-embed-text:latest",
            prompt: text
        })
        .send()
        .await?
        .json::<EmbeddingResponse>()
        .await?;
    Ok(res.embedding)
}