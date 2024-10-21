
use core::str;
use std::{error::Error, path::Path, process::Command};
use redis::Commands;
extern crate redis;
use crate::utils::gpt::prompt_gpt;
use std::fs;

fn get_changes(directory: &str) -> String {
    let mut file_args = Vec::new();
    let extensions = vec!["rs", "py", "js", "cpp", "java", "c", "ts", "go", "rb", "php","tsx","jsx"];
    
    for entry in fs::read_dir(directory).expect("Directory read failed") {
        let entry = entry.expect("Failed to get directory entry");
        if let Some(ext) = entry.path().extension() {
            if extensions.contains(&ext.to_str().unwrap()) {
                file_args.push(entry.path().display().to_string());
            }
        }
    }

    println!("{:?}", file_args);


    let diff = Command::new("git")
        .arg("diff")
        .args(&file_args)
        .output()
        .expect("Failed to execute diff");

    let mut changes = String::from_utf8_lossy(&diff.stdout).to_string();

    let status = Command::new("git")
        .arg("status")
        .arg("--short")
        .output()
        .expect("Failed to execute git status");

    let status_string = String::from_utf8_lossy(&status.stdout).to_string();


    changes.push_str("\nUntracked files:\n");
    for line in status_string.lines() {
        if line.starts_with("??") {
            let file_path = line[3..].trim(); 
            if let Some(ext) = Path::new(file_path).extension() {
                if extensions.contains(&ext.to_str().unwrap()) {
                    if let Ok(file_contents) = fs::read_to_string(file_path) {
                        changes.push_str(&format!("\n{}:\n{}\n", file_path, file_contents));
                    } else {
                        changes.push_str(&format!("\n{}: (Could not read file)\n", file_path));
                    }
                }
            }
        }
    }

    println!("{}", changes);

    return changes
}


fn add_feedback(val: &str) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let key: i32 = con.incr("feedback_counter", 1)?;

    let full_key = format!("{}larn", key);

    let _: () = con.set(&full_key, val)?;
    Ok(full_key)
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



pub async fn add(directory:&str) -> Result<(), Box<dyn Error>> {
    let changes = get_changes(&directory);
    let feedback = get_feedback(&changes).await?;
    add_feedback(&feedback)?;
    print!("{:?}",feedback);

    Ok(())

}
