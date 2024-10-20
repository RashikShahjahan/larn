
use core::str;
use std::process::Command;
use redis::Commands;
use reqwest::Client;
use std::env;
use serde_json::json;
use std::error::Error;
use serde_json::Value;
extern crate redis;

fn get_changes()->String{
    //Here you go
    let diff = Command::new("git").arg("diff").arg("--").arg("*.rs").output().expect("Failed to execute diff");
    Command::new("git").arg("add").arg(".").status().expect("Failed to execute add");
    let diff_string = str::from_utf8(&diff.stdout)
        .expect("Failed to convert output to string")
        .to_string();

    return diff_string;
}

fn add_feedback(key:i32,val:&str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set(key,val)?;

    Ok(())
}



#[tokio::main] 
async fn get_feedback(changes:&str,key:i32) -> Result<(), Box<dyn Error>> {
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let client = Client::new();

    let request_body = json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": "You are a highly skilled code reviewer. You will analyze the following git diffs to find mistakes and areas of improvement. Do not write any code.Make your suggestions concise."
            },
            {
                "role": "user",
                "content": changes
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
                let _ = add_feedback(key,message);
            }
        }
    }

    Ok(())


}


pub fn add(key:i32){
    let changes = get_changes();
    let _ = get_feedback(&changes,key);
}
