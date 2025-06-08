use reqwest::Client;
use tokio_stream::StreamExt;
use serde_json::{Value, json};
use std::io::{stdout, Stdout, Write};

pub async fn answer_with_context(
    posts: Vec<String>,
    question: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut context = String::new();

    // prepare the prompt using the context and reddit post content
    for (i, string) in posts.iter().enumerate() {
        context.push_str(format!("{}\n{}\n\n", i, string).as_str());
    }

    let prompt = format!(
        "Use the following reddit posts to answer the user's questions, which are numbered and separated by two new line chars:\n{}\n\nQuestion: {}",
        &context, question
    );

    let client = Client::new();
    let request_body = json!({
        "model": "llama3.2",
        "messages":[{
            "role": "user",
            "content" : &prompt
        }],
        "stream": true
    });
    
    // start ollama chat 
    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&request_body)
        .send()
        .await?;

    let mut response_stream = res.bytes().await?;
    let mut final_response = String::new();

    // handle response stream and give a real time typing effect
    while let Some(item) = response_stream.into_iter().next() {
        match item {
            Ok(chunk) => {
                let text = String::from_utf8_lossy(chunk);
                for line in text.lines(){
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if let Some(content) = json["message"]["content"].as_str() {
                            print!("{}", &content);
                            stdout().flush()?;
                            final_response.push(content);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Stream error : {}", e);
            }
        }
    }

    Ok(final_response)
}
