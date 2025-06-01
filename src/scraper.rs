use reqwest::Client;
use std::env;
use tokio::{fs, fs::File};
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

#[derive(Debug)]
pub struct Post{
    pub id: String,
    pub title: String,
    pub self_text: String,
    pub author: String,
    pub upvotes: i64,
    pub downvotes: i64,
    pub url: String
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
    file.write_all(response.as_bytes()).await?;

    println!("Response written to file successfully");
    Ok(())
}

pub async fn prepare_posts() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("reddit_posts.json").await?;
    let json: Value = serde_json::from_str(&contents)?;
    let mut post_list: Vec<Post> = Vec::new();
    if let Some(posts) = json["data"]["children"].as_array() {
        for post in posts {
            let data = &post["data"];
            let id = data["id"].as_str().unwrap_or("No id given").to_string();
            let title = data["title"].as_str().unwrap_or("No title").to_string();
            let author = data["author"].as_str().unwrap_or("unknown").to_string();
            let score = data["score"].as_i64().unwrap_or(0);
            let downvotes = data["down"].as_i64().unwrap_or(0);
            let content = data["selftext"].as_str().unwrap_or("no content").to_string();
            let url = data["url"].as_str().unwrap_or("url not found").to_string();

            post_list.push(Post { id, title, self_text: content, author, upvotes: score, downvotes, url });
            
        }
    } else {
        println!("unexpected json format");
    }
    Ok(post_list)
}