use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::env::{set_current_dir, current_dir};

fn parse_line(line: &str) -> Vec<String> {
    let mut is_quote = false;
    let mut token = String::new();
    let mut tokens = Vec::new();
    for c in line.chars() {
       if c == '"'  {
        is_quote = !is_quote;
       } else if c.is_whitespace() && !is_quote  {
           tokens.push(token.clone());
           token.clear();
       } else {
        token.push(c)
       }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    
    tokens
}

fn read_command() -> io::Result<Vec<String>> {
    print!("$ ");
    io::stdout().flush()?;

    let mut command = String::new();
    io::stdin().read_line(&mut command)?;

    let v = parse_line(&command);
    Ok(v)
}

fn main() {
    loop {
        let c = read_command().expect("la lecture peut echouer (ca arrive rarement)");
        let Some((command, args)) = c.split_first() else {
            continue;
        };

        if command == "exit" {
            break;
        }

        if command == "cd" {
            let path = match args.first() {
            Some(dir) => PathBuf::from(dir),
            None => match std::env::home_dir() {
                Some(home) => home,
                None => {
                    println!("...");
                    continue;
                }
            },
        };
            let Ok(()) = set_current_dir(path) else { println!("Invalid path"); continue };
            continue;
        }

        if command == "pwd" {
            match current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("ERROR: {}", e),
            }
            continue;
        }

        let command_result = Command::new(command).args(args).status();
        match command_result {
            Ok(success) => println!("{}", success),
            Err(e) => println!("{}", e),
        }
    }
}

/*
is_guillemet = false
pour chaque caractère c :
    si c == guillemet :
        is_guillemet = !is_guillemet
    sinon si c == espace ET pas dans un guillemet :
        
        tokens.push(token)
        token.clear()
    sinon :
        token.push(c)
*/

