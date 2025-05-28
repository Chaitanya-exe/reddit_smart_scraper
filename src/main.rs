use embedding::get_embedding;

mod store;
mod scraper;
mod chunks;
mod embedding;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let token = scraper::get_access_token().await?;
    // let posts = scraper::get_subreddit_top(&token).await?;
    // scraper::put_into_file(posts).await?;
    // scraper::parse_json().await?;

    store::create_collection("reddit_posts", 768).await?;
    Ok(())
}
