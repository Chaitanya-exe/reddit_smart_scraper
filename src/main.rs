mod store;
mod scraper;
mod chunks;
mod embedding;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = scraper::get_access_token().await?;
    let posts = scraper::get_subreddit_top(&token).await?;
    
    scraper::put_into_file(posts).await?;
    let posts = scraper::prepare_posts().await?;
    
    for post in posts{

    }
 

    // store::create_collection("reddit_posts", 768).await?;
    Ok(())
}
