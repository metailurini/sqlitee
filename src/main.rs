use std::io::{stdin, stdout, Write};
use std::process::exit;

const EXIT_SUCCESS: i32 = 0;

#[allow(dead_code)]
enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum StatementType {
    StatementNone,
    StatementInsert,
    StatementSelect,
}

struct Statement {
    s_type: StatementType,
}

struct InputBuffer {
    pub buffer: Option<String>,
    pub buffer_length: usize,
    pub input_length: Option<isize>,
    pb: String,
}

impl InputBuffer {
    fn new() -> Self {
        Self {
            buffer: None,
            buffer_length: 0,
            input_length: None,
            pb: String::new(),
        }
    }

    fn read_input(&mut self) {
        print_prompt();
        stdout().flush().expect("Failed to flush stdout");

        self.pb.clear();
        let input = match stdin().read_line(&mut self.pb) {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                exit(1);
            }
        };
        self.buffer = Some(self.pb.clone());
        self.buffer_length = self.pb.capacity();
        self.input_length = Some(input as isize - 1);
        self.buffer.as_mut().unwrap().pop();
    }

    /*
        fn close(&mut self) {
            self.buffer = None;
            self.buffer_length = 0;
            self.input_length = None;
            self.pb.clear();
        }
    */
}

fn print_prompt() {
    print!("db > ");
}

fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    let inp = match input_buffer.buffer.clone() {
        None => return MetaCommandResult::MetaCommandUnrecognizedCommand,
        Some(inp) => inp,
    };

    if inp == ".exit" {
        exit(EXIT_SUCCESS)
    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}

fn prepare_statement(input_buffer: &InputBuffer, stm: &mut Statement) -> PrepareResult {
    let inp = match input_buffer.buffer.clone() {
        None => return PrepareResult::MetaCommandUnrecognizedCommand,
        Some(inp) => inp.clone(),
    };
    let i = inp.trim();

    if i == "select" {
        stm.s_type = StatementType::StatementSelect;
        return PrepareResult::MetaCommandSuccess;
    }

    if i == "insert" {
        stm.s_type = StatementType::StatementInsert;
        return PrepareResult::MetaCommandSuccess;
    }

    if i == "" {
        return PrepareResult::MetaCommandSuccess;
    }

    return PrepareResult::MetaCommandUnrecognizedCommand;
}

fn execute_statement(stm: &Statement) {
    match stm.s_type {
        StatementType::StatementNone => {}

        StatementType::StatementSelect => {
            println!("this is the select statement")
        }

        StatementType::StatementInsert => {
            println!("this is the insert statement")
        }
    }
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        input_buffer.read_input();

        let buffer = input_buffer.buffer.to_owned().unwrap();
        let head_char = match buffer.chars().nth(0) {
            Some(head_char) => head_char,
            None => '\0',
        };
        if head_char == '.' {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::MetaCommandSuccess => {
                    continue;
                }

                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!(
                        "Unrecognized command '{}'",
                        input_buffer.buffer.as_ref().unwrap()
                    );
                    continue;
                }
            }
        }

        let mut statement: Statement = Statement {
            s_type: StatementType::StatementNone,
        };
        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::MetaCommandSuccess => {}
            PrepareResult::MetaCommandUnrecognizedCommand => {
                println!(
                    "Unrecognized keyword at start of '{}'.",
                    input_buffer.buffer.as_ref().unwrap()
                );
                continue;
            }
        }
        execute_statement(&statement);
    }
}
