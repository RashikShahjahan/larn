
use std::process::{Command, Output};

fn get_changes()->Output{
    let diff = Command::new("git").arg("diff").output().expect("Failed to execute diff");
    Command::new("git").arg("add").arg(".").status().expect("Failed to execute add");

    return diff;

}

fn get_feedback(){

}

pub fn add(){
    println!("{:?}",get_changes());
}
