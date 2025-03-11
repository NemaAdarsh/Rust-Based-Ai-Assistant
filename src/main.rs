mod ai;

use std::io::{self, Write};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    print!("Enter your Hugging Face API key: ");
    io::stdout().flush().unwrap();

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).unwrap();
    let api_key = api_key.trim().to_string();  // this is to trim the newlines 

    println!("Welcome to AI Code Assistant! Type code and get AI suggestions.");
    println!("Type your code, then type 'END' on a new line to submit.");

    loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();

        //collects more than one till till the user types END
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if line.trim() == "END" {
                break;
            }
            input.push_str(&line);
        }

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let suggestion = rt.block_on(ai::generate_suggestion(&api_key, input));
        println!("\nAI Suggestion:\n{}\n", suggestion);
    }
}
