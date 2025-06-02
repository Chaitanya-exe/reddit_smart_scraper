use reqwest::Client;
use uuid::Uuid;
use serde_json::{self, json, Value};
use std::env;

use crate::scraper::Post;
use crate::embedding::Embeddings;

pub struct VectorResponse{

}

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

pub async fn upsert_vector(embeddings: Embeddings, id: String, post_struct: &Post) -> Result<(), Box<dyn std::error::Error>> {
    //request initializezrs
    let client = Client::new();
    let db_url = format!("http://localhost:6333/collections/{}/points/search", env::var("COLLECTION_NAME").unwrap());
    let request_body = json!({
        "vector": &embeddings.points,
        "limit":1,
        "with_payload":true,
        "filter":{
            "must":[{
                "key":"post_id",
                "match":{ "value": &id}
            }]
        }
    });

    //database api request
    let db_response = client.post(&db_url)
        .json(&request_body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    //resutl validatiaon    
    if let Some(arr) = db_response["result"].as_array() {
        if !arr.is_empty() {
            println!("⚠️ Post ID {} already exists in vector DB", &id);
            return Ok(());
        }
    }

    //insert new vector in case of new post
    let auto_id = Uuid::new_v4().to_string();
    let insert_body = json!({
        "points":[{
            "id": &auto_id,
            "vector": &embeddings.points,
            "payload":{
                "post_id": &id,
                "title": &post_struct.title,
                "self_text" : &post_struct.self_text,
                "upvotes": &post_struct.upvotes,
                "downvotes" : &post_struct.downvotes,
                "author" : &post_struct.author,
                "url" : &post_struct.url
            }
        }]
    }); 

    let url = format!("http://localhost:6333/collections/{}/points", env::var("COLLECTION_NAME").unwrap());
    let insert_response = client.put(&url)
        .json(&insert_body)
        .send()
        .await?;

    if insert_response.status().is_success() {
        println!("✅ Stored vector for post {}", post_struct.id);
        println!("{}", insert_response.text().await?);
    } else {
        println!("❌ Failed to store post: {:?}", insert_response.text().await?);
    }
    Ok(())  
}

pub async fn search_similar_vectors(query: Vec<f32>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let search_body = json!({
        "vector" : query,
        "limit": 10,
        "with_payload": true
    });
    let url = format!("{}collections/{}/points/search", env::var("BASE_URL").unwrap(), env::var("COLLECTION_NAME").unwrap());
    let response = client.post(&url)
        .json(&search_body)
        .send()
        .await?
        .text()
        .await?;

    let parsed = serde_json::from_str::<Value>(&response).unwrap();
    let texts = parsed["result"].as_array().unwrap().iter()
        .map(|r| {
            if let Some(res) = r["payload"]["self_text"].as_str() {
                return res.to_string()
            } else{
                return String::from("No value")
            }
        })
        .collect();
    Ok(texts)
}