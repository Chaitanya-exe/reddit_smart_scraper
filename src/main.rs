use embedding::get_embedding;

mod scraper;
mod chunks;
mod embedding;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let token = scraper::get_access_token().await?;
    // let posts = scraper::get_subreddit_top(&token).await?;
    // scraper::put_into_file(posts).await?;
    // scraper::parse_json().await?;

    let text = "Reddit is a network of communities where people can dive into their interests, hobbies and passions.";
    let chunked_data = chunks::chunk_text(&text, 20);

    for chunk in chunked_data{
        let embeddings = get_embedding(&chunk).await?;
        println!("chunk: {:?}\nEmbedding: {:?}", chunk, embeddings);
        println!("============================");
    }
    Ok(())
}
