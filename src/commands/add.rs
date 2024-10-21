
use core::str;
use std::{error::Error, process::Command};
use redis::Commands;
extern crate redis;

use crate::utils::gpt::prompt_gpt;
fn get_changes()->String{
    let diff = Command::new("git")
    .arg("diff")
    .arg("--")
    .arg("*.rs")
    .arg("*.py")
    .arg("*.js")
    .arg("*.cpp")
    .arg("*.java")
    .arg("*.c")
    .arg("*.ts")
    .arg("*.go")
    .arg("*.rb")
    .arg("*.php")
    .output()
    .expect("Failed to execute diff");

    Command::new("git").arg("add").arg(".").status().expect("Failed to execute add");
    let diff_string = str::from_utf8(&diff.stdout)
        .expect("Failed to convert output to string")
        .to_string();

    return diff_string;
}

fn add_feedback(val:&str) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let key: i32 = con.incr("feedback_counter", 1)?;

    let _: () = con.set(key.to_string(), val)?;
    Ok(key.to_string())
}


async fn get_feedback(changes: &str)-> Result<String, Box<dyn Error>> {
    let system_prompt = "Please review the following code changes. Consider:
                        1. Code quality and adherence to best practices.
                        2. Potential bugs or edge cases.
                        3. Performance optimizations.
                        4. Readability and maintainability.
                        5. Any security concerns. 
                        Do not write any code or provide improvements. Just look for flaws.";
    
                        let response = prompt_gpt(system_prompt, changes, "gpt-4o").await?;
                        Ok(response)
}



pub async fn add() -> Result<(), Box<dyn Error>> {
    let changes = get_changes();
    let feedback = get_feedback(&changes).await?;
    add_feedback(&feedback)?;
    print!("{:?}",feedback);

    Ok(())

}
