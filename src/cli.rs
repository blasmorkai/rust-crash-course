use std::env;

pub fn run() {
    println!("\nCLI Section");
    
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command = args[1].clone();
        let name = "Brad";
        let status = "100%";

        println!("Command: {:?}", command);

        if command == "hello" {
            println!("Hi {}, how are you?", name);
        } else if command == "status" {
            println!("Status is {}", status);
        } else {
            println!("That is not a valid command");
        }
    } else {
        println!("No command has been provided");
    }



}