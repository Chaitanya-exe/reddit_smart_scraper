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
        let post_ref = &post;
        let post_chunk = chunks::chunk_text(&post, 128);

        for (i, chunk) in post_chunk.iter().enumerate() {
            let chunk_id = format!("{}_chunk_{}", &post_ref.id , i);
            let embedding_struct = embedding::get_embedding(&chunk).await?;

            store::upsert_vector(embedding_struct, chunk_id, post_ref).await?;
        }
        
    }
 
    Ok(())
}
