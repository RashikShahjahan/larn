use clap::Parser;
mod commands;
mod utils;

#[derive(Parser)]
struct Cli {
    command:String,
}

#[tokio::main]
async fn main() {
    /*	
        Step 1:
        Create empty cli commands: larn init, larn add, and larn learn [Done]
        Step 2
        Implement helper to store code changes ->git diff and git add .[Done]
        Step 3
        Implement helper to make LLM calls[Done]
        Step 4
        Implement helper to write to Redis[Done]
        Step 5
        Implement function to read all feedback in Redis and pass them to LLM for finding trends in coding mistakes and giving excercises to fix those issues[Done]
        Step 6
        Deploy rust package
    */
    
    let args = Cli::parse();
   if args.command == "add"{
        let _ = commands::add::add().await;
    }
    else if args.command == "learn"{
        let output = commands::learn::learn().await;
        print!("{:?}",output);
    }
    else{
        println!("{} is not a valid command" , args.command);
    }

}
