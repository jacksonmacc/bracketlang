use std::io::{Write, stdin, stdout};

use reader::{DataType, ParseError, Reader, get_regex, tokenize};

mod reader;

#[cfg(test)]
mod tests;

fn read(input: String) -> Result<DataType, ParseError> {
    let tokens = tokenize(input, get_regex());
    let mut reader = Reader::new(tokens);
    reader.read()
}

fn eval(input: DataType) -> DataType {
    input
}

fn print(input: DataType) -> String {
    format!("{:?}", input)
}

fn rep(input: String) -> String {
    let read_result = match read(input) {
        Ok(r) => r,
        Err(e) => return e.msg,
    };

    let eval_result = eval(read_result);
    let print_result = print(eval_result);
    print_result
}

fn main() {
    loop {
        print!("user> ");
        stdout()
            .flush()
            .expect("Flushing stdout should have worked.");
        let mut user_input = String::new();

        let user_input_result = stdin()
            .read_line(&mut user_input)
            .expect("Didn't enter a correct string");

        // Recieved EOF
        if user_input_result == 0 {
            break;
        }

        println!("{}", rep(user_input))
    }
}
