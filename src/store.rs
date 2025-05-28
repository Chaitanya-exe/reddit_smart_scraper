use reqwest::{Client, StatusCode};
use serde_json::{self, json};

pub async fn create_collection(name: &str, size: usize) -> Result<(), Box<dyn std::error::Error>>{
    let client = Client::new();
    let url = format!("http://localhost:6333/collections/{}", name);
    let body = json!({
        "vectors":{
            "size": size,
            "distance": "cosine"
        }
    });

    let response = client.put(&url).json(&body).send().await?;
    println!("{}",response.status().as_u16());
    if response.status().is_success(){
        println!("collection created: {}", name);
        return Ok(())
    }
    println!("Collection named: {} already exists!!", name);

    Ok(())
}

pub async fn upsert_vector(vector: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    Ok(())
}