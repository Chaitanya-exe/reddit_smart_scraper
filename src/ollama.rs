use reqwest::Client;
use serde_json::{Value, json};

pub async fn answer_with_context(
    posts: Vec<String>,
    question: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut context = String::new();

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
        }]
    });

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&request_body)
        .send()
        .await?
        .text()
        .await?;

    let preview_len = std::cmp::min(200, res.len());

    println!(
        "Response preview: (first {} characters): {}",
        preview_len,
        &res[..preview_len]
    );

    if res.contains('\n') {
        let mut final_content = String::new();

        for line in res.lines() {
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Ok(json_value) => {
                    if let Some(content) = json_value
                        .get("message")
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                    {
                        final_content = content.to_string();
                    }
                }
                Err(e) => {
                    println!("Failed to parse line as JSON: {}", e);
                    println!("Problematic line : {}", line);
                }
            }
        }

        if !final_content.is_empty() {
            Ok(final_content)
        } else {
            Err(format!("failed to extractt any content").into())
        }
    } else {
        match serde_json::from_str::<Value>(&res) {
            Ok(json_value) => {
                match json_value
                    .get("message")
                    .and_then(|m| m.get("content"))
                    .and_then(|c| c.as_str())
                {
                    Some(content) => Ok(content.to_string()),
                    None => Err(format!("Missings Expected fields in JSON").into()),
                }
            }
            Err(e) => Err(format!("Failed to parse the json response: {e}").into()),
        }
    }
}
