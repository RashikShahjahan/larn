use std::env;
use std::error::Error;

use redis::Commands;
use reqwest::Client;
use serde_json::{json, Value};

fn retrieve_feedback() -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let keys: Vec<String> = con.keys("*")?;
    let mut result = String::new();

    for key in keys {
        let value: String = con.get(&key)?;
        result.push_str(&value);   
        con.del(&key)?; 
    }

    Ok(result)
}

#[tokio::main] 
async fn get_detailed_feedback(feedback:String) -> Result<(), Box<dyn Error>> {
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let client = Client::new();

    let request_body = json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": "You an expert in analyzing code reviews. You will read the list of reviews and identify upto 3 things software engineer is most struggling with and create a set of programming challenges to help them improve on those areas"
            },
            {
                "role": "user",
                "content": feedback
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

    let response_json:Value = response.json().await?;

    if let Some(choices) = response_json["choices"].as_array() {
        if let Some(choice) = choices.get(0) {
            if let Some(message) = choice["message"]["content"].as_str() {
                println!("Assistant's response: {}", message);
            }
        }
    }

    Ok(())


}


pub fn learn(){
    let  result = retrieve_feedback();
    let _ = get_detailed_feedback(result.unwrap());
}
