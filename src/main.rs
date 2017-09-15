use std::io::{self, Write};

fn prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer.pop();
            buffer
        }
        Err(error) => {
            println!("error: {}", error);
            ::std::process::exit(1);
        }
    }
}

fn main() {
    loop {
        prompt();
        let input = read_input();

        match input.as_ref() {
            ".exit" => ::std::process::exit(0),
            _ => println!("Unrecognized command: {}", input)
        }
    }
}
