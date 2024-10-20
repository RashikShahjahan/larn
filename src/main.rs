use clap::Parser;
mod commands;

#[derive(Parser)]
struct Cli {
    command:String,
}


fn main() {
    /*	
        Step 1:
        Create empty cli commands: larn init, larn add, and larn learn [Done]
        Step 2
        Implement helper to store code changes ->git diff and git add .[Done]
        Step 3
        Implement helper to make LLM calls and store responses in file
        Step 4
        Implement larn add
        Step 5 
        Implement larn learn
        Step 6
        Implement larn init
        Step 7
        Deploy rust package
    */
    
    let args = Cli::parse();
    if args.command == "init"{
        println!("larn init");
    }
    else if args.command == "add"{
        println!("larn add");
        commands::add::add();
    }
    else if args.command == "learn"{
        println!("larn learn");
    }
    else{
        println!("{} is not a valid command" , args.command);
    }

}
