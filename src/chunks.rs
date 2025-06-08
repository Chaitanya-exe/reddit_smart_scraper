use crate::scraper::Post;


// chunks the data for vector preparation.
pub fn chunk_text(post: &Post, max_words: usize) -> Vec<String> {
    let combined = format!("ID: {}\nTitle: {}\nAuthor: {}\nContent: {}\nUpvotes: {}\nDownvotes: {}\nurl: {}",
        post.id,
        post.title,
        post.author,
        post.self_text,
        post.upvotes,
        post.downvotes,
        post.url
    );

    let words: Vec<&str> = combined.split_whitespace().collect();
    words
        .chunks(max_words)
        .map(|chunk| chunk.join(" "))
        .collect()
}