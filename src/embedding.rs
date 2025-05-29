use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest <'a>{
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
pub struct EmbeddingResponse{
    pub points: Vec<f32>
}

pub struct Embeddings{
    pub points: Vec<f32>,
    pub payload: String
}

pub async fn get_embedding(text: &str) -> Result<Embeddings, Box<dyn std::error::Error>>{
    let client = Client::new();
    let mut response_embedding = Embeddings{points: vec![], payload: text.to_string()};
    let res = client.post("http://localhost:11434/api/embeddings")
        .json(&EmbeddingRequest{
            model: "nomic-embed-text:latest",
            prompt: text
        })
        .send()
        .await?
        .json::<EmbeddingResponse>()
        .await?;
    response_embedding.points = res.points;
    Ok(response_embedding)
}