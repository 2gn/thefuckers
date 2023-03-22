use std::env;
// use std::str;

fn is_perfect_command(command: &str) -> Result<(), ()> {
    if command == "ls" {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    if let Ok(shell_history) = env::var("TF_HISTORY") {
        let shell_history = shell_history.split("\n").collect::<Vec<&str>>()[0];
        match is_perfect_command(shell_history) {
            Ok(_) => {
                println!("No fucks given");
            },
            Err(_) => {
                println!("Problems");
            }
        }

    } else {
        println!("environment variable TF_HISTORY is not set");
    }
}
