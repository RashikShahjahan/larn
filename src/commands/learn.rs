use std::error::Error;
use crate::utils::gpt::prompt_gpt;

use redis::Commands;



fn retrieve_feedback() -> Result<String, Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let mut cursor = 0;
    let mut result = String::new();

    loop {
        let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .cursor_arg(cursor)
            .arg("MATCH")
            .arg("*")
            .query(&mut con)?;

        for key in keys {
            let value: String = con.get(&key)?;
            result.push_str(&value);
            con.del(&key)?;
        }

        cursor = next_cursor;
        if cursor == 0 {
            break;
        }
    }

    Ok(result)
}


async fn get_detailed_feedback(feedback: &str)-> Result<String, Box<dyn Error>> {
    let system_prompt = "Please review this list of flaws in a software engineers code changes and identify upto 3 things software engineer is most struggling with. Create a set of programming challenges to help them improve on those areas";
    
                        let response = prompt_gpt(system_prompt, feedback, "gpt-4o").await?;
                        Ok(response)
}





pub async  fn learn()-> Result<String, Box<dyn Error>> {
    let  result = retrieve_feedback()?;
    let detailed_feedback= get_detailed_feedback(&result).await?;
    Ok(detailed_feedback)

}
