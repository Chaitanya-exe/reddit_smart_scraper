use reqwest::Client;
use serde_json::{self, json};

pub async fn create_collection(name: &str, size: usize) -> Result<(), Box<dyn std::error::Error>>{
    let client = Client::new();
    let url = format!("http://localhost:6333/collections/{}", name);
    let body = json!({
        "vectors":{
            "size": size,
            "distance": "Cosine"
        }
    });

    let response = client.put(&url).json(&body).send().await?;
    
    if response.status().is_success(){
        println!("collection created: {}", name);
        return Ok(())
    }
    println!("Collection named: {} already exists!!", name);

    Ok(())
}

pub async fn upsert_vector(vector: Vec<f32>, id: String, payload: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    Ok(())
}