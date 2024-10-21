use std::{env, error::Error};
use reqwest::Client;
use serde_json::{json, Value};

pub async fn prompt_gpt(system_prompt: &str, user_prompt: &str, model: &str) -> Result<String, Box<dyn Error>> {
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let client = Client::new();

    let request_body = json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": user_prompt
            }
        ]
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&request_body)
        .send()
        .await?; 

    let response_json: Value = response.json().await?;

    let message = response_json.get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|array| array.get(0))
        .and_then(|first| first.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str());

    match message {
        Some(m) => Ok(m.to_string()),
        None => Err("No message found in the response".into())
    }
}
