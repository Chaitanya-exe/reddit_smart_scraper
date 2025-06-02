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

pub struct Embeddings{
    pub points: Vec<f32>,
}

pub async fn get_embedding(text: &str) -> Result<Embeddings, Box<dyn std::error::Error>>{
    let client = Client::new();
    let mut response_embedding = Embeddings{points: vec![]};
    let res = client.post("http://localhost:11434/api/embeddings")
        .json(&EmbeddingRequest{
            model: "nomic-embed-text:latest",
            prompt: text
        })
        .send()
        .await?
        .json::<EmbeddingResponse>()
        .await?;
    
    response_embedding.points = res.embedding;
    println!("embeddings prepared successfully");

    Ok(response_embedding)
}