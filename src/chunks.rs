pub fn chunk_text(text: &str, max_words: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    words
        .chunks(max_words)
        .map(|chunk| chunk.join(" "))
        .collect()
}