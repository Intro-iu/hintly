mod utils;
mod client;
use std::env;
use std::io::Write;
use clap::{Arg, Command};
use clipboard_rs::{Clipboard, ClipboardContext};

fn run_operation(operation: &str, command: &str) {
    if operation == "r" || operation == "run" {
        let os = env::consts::OS;

        let shell; 
        let run_flag;

        if os == "windows" {
            shell = "powershell";
            run_flag = "-Command";
        } else {
            shell = "sh";
            run_flag = "-c";
        }

        let output = std::process::Command::new(shell)
            .arg(run_flag)
            .arg(command)
            .output()
            .expect("Failed to execute command");
        
        if output.status.success() {
            println!("Command executed successfully.");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error: {}", stderr);
        }
    } else if operation == "c" || operation == "copy" {
        let ctx = ClipboardContext::new().unwrap();
        ctx.set_text(command.to_string()).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let config = utils::read_config().unwrap();
    let api_key = config.api_key;
    let base_url = config.base_url;


    let client = client::Client::new(api_key, base_url);

    let model = "qwen-max";

    let matches = Command::new("hintly")
        .version("0.1.0")
        .author("Intro <Intro-iu@outlook.com>")
        .about("A simple command line tool in Rust inorder to simplify the process of running commands in the terminal")
        .arg(Arg::new("prompt")
                .help("The requirement from user")
                .required(true)
                .index(1))
        .arg(Arg::new("operation")
                .help("The operation you want to perform")
                .required(false)
                .index(2))
        .get_matches();

    if let Some(prompt) = matches.get_one::<String>("prompt") {
        let command = client.chat(model, prompt).await.unwrap();
        println!("Hint: {}", command);

        if let Some(operation) = matches.get_one::<String>("operation") {
            run_operation(&operation, &command);
        } else {
            print!("Enter your operation [r(un) | c(opy) | q(uit)]: ");
            let mut operation = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut operation).unwrap();
            let operation = operation.trim();

            run_operation(&operation, &command);
        }
    } 
}
