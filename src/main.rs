use std::io::{stdin, stdout, Write};
use std::process;

fn main() {
    let mut input_buffer: String = String::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer);

        match input_buffer.as_str() {
            ".exit" => process::exit(0),
            ".foo" => println!("Entered foo"),
            _ => {
                println!("Unknown command entered")
            }
        }
    }
}

fn read_input(input_buffer: &mut String) {
    let _ = stdout().flush();
    input_buffer.clear();

    stdin()
        .read_line(input_buffer)
        .expect("Did not enter a valid str");

    if let Some('\n') = input_buffer.chars().next_back() {
        input_buffer.pop();
    }

    if let Some('\r') = input_buffer.chars().next_back() {
        input_buffer.pop();
    }
}

fn print_prompt() {
    print!("db > ");
}
