use reqwest::Client;
use std::env;
use tokio::{fs, fs::File, io::AsyncReadExt};
use tokio::io::AsyncWriteExt;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct AuthResponse{
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

pub async fn get_access_token() -> Result<String, Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();
    let client = Client::new();

    let res = client.post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(env::var("CLIENT_ID")?, Some(env::var("CLIENT_SECRET")?))
        .form(&[
            ("grant_type", "password"),
            ("username", &env::var("USERNAME")?),
            ("password", &env::var("PASSWORD")?)
        ])
        .header("User-Agent", format!("rust:simple-scraper:v1.0 (by /u/{})",env::var("USERNAME")?))
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;
    
    println!("access token granted successfully");
    Ok(res.access_token)

}

pub async fn get_subreddit_top(access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://oauth.reddit.com/r/rust/top?limit=5")
        .bearer_auth(access_token)
        .header("User-Agent",format!("rust:simple-scraper:v1.0 (by /u/{})",env::var("USERNAME")?))
        .send()
        .await?
        .text()
        .await?;

    println!("Scraped posts successfully");
    Ok(res)
}


pub async fn put_into_file(response: String) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::create("reddit_posts.json").await?;
    file.write_all(serde_json::to_string_pretty(&response)?.as_bytes()).await?;

    println!("Response written to file successfully");
    Ok(())
}

pub async fn parse_json() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("reddit_posts.json").await?;
    let json: Value = serde_json::from_str(&contents)?;

    if let Some(posts) = json["data"]["children"].as_array() {
        for post in posts {
            let data = &post["data"];
            let title = data["title"].as_str().unwrap_or("No title");
            let author = data["author"].as_str().unwrap_or("unknown");
            let score = data["score"].as_i64().unwrap_or(0);

            println!("Title: {}", title);
            println!("author: {}", author);
            println!("Score: {}", score);
            println!("===========================");
        }
    } else {
        println!("unexpected json format");
    }
    Ok(())
}