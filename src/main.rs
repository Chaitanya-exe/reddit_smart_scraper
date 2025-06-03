mod store;
mod scraper;
mod chunks;
mod embedding;
mod ollama;


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

    let user_question = "What are these posts talking about?";

    let query_vec = embedding::get_embedding(user_question).await?;

    let search_result = store::search_similar_vectors(&query_vec.points, 5).await?;

    let chat_response = ollama::answer_with_context(search_result, user_question.to_string()).await?;
    
    println!("Got response from ollama: \n{}", chat_response);
    Ok(())
}