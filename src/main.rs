use std::io::{stdin, stdout, Write};
use std::process;

// TODO(iddamalm): This could be an Optional
enum MetaCommandResult {
    Success,
    Unrecognised,
}

enum PrepareResult {
    Success,
    Unrecognised,
    // SyntaxError
    // NegativeId,
    // StringTooLong,
}

pub enum StatementType {
    Ignored,
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
}

fn main() {
    let mut input_buffer: String = String::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer);

        // Check for meta command
        if input_buffer.starts_with(".") {
            match meta_command(&input_buffer) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::Unrecognised => {
                    println!("Unrecognised command: {}", &input_buffer);
                    continue;
                }
            }
        }

        let (result, statement) = prepare_statement(&input_buffer);

        match result {
            PrepareResult::Success => {}
            PrepareResult::Unrecognised => {
                println!("Unrecognised statement: {}", &input_buffer);
                continue;
            }
        }

        exec_statement(&statement);
        println!("Executed {}", &input_buffer);
    }
}

fn meta_command(input_buffer: &String) -> MetaCommandResult {
    match input_buffer.as_str() {
        ".exit" => process::exit(0),
        ".foo" => {
            println!("Entered foo");
            MetaCommandResult::Success
        }
        _ => {
            println!("Unknown command entered: {}", &input_buffer);
            MetaCommandResult::Unrecognised
        }
    }
}

fn prepare_statement(input_buffer: &String) -> (PrepareResult, Statement) {
    if input_buffer.starts_with("insert") {
        let s = Statement {
            statement_type: StatementType::Insert,
        };
        return (PrepareResult::Success, s);
    }

    if input_buffer.starts_with("select") {
        let s = Statement {
            statement_type: StatementType::Select,
        };
        return (PrepareResult::Success, s);
    }

    (
        PrepareResult::Unrecognised,
        Statement {
            statement_type: StatementType::Ignored,
        },
    )
}

fn exec_statement(statement: &Statement) {
    match statement.statement_type {
        StatementType::Insert => {
            println!("This is where we insert")
        }
        StatementType::Select => {
            println!("This is where we select")
        }
        StatementType::Ignored => {}
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
    print!("\ndb > ");
}
