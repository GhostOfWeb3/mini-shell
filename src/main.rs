use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::process::Command;

fn main() {
    let mut rl = DefaultEditor::new().expect("Failed to create editor");

    loop {
        let cwd = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| String::from("?"));

        let prompt = format!("{} $ ", cwd);

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }
                rl.add_history_entry(&input).ok();
                handle_input(&input);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }
}

fn handle_input(input: &str) {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command = parts[0];
    let args = if parts.len() > 1 { parts[1] } else { "" };

    match command {
        "exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        "help" => {
            println!("Built-in commands:");
            println!("  cd <path>   - Change directory");
            println!("  help        - Show this help message");
            println!("  exit        - Exit the shell");
            println!("Any other input is run as an external command.");
        }
        "cd" => {
            if args.is_empty() {
                eprintln!("cd: missing argument");
            } else {
                if let Err(e) = env::set_current_dir(args) {
                    eprintln!("cd: {e}");
                }
            }
        }

        "ls" => {
            let status = Command::new("cmd")
                .args(["/C", "dir", args])
                .status();

            match status {
                Ok(s) if !s.success() => {
                    eprintln!("Process exited with status: {s}");
                }
                Err(e) => {
                    eprintln!("ls: failed to execute ({e})");
                }
                _ => {}
            }
        }

        _ => {
            let status = Command::new("cmd")
                .args(["/C", input])
                .status();

            match status {
                Ok(s) if !s.success() => {
                    eprintln!("Process exited with status: {s}");
                }
                Err(e) => {
                    eprintln!("{command}: command not found ({e})");
                }
                _ => {}
            }
        }
    }
}