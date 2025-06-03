use std::{
    io::{Write, stdin, stdout},
    rc::Rc,
};

use env::{
    ADDITION, DIVISION, EQUALS, GREATER_THAN, GREATER_THAN_OR_EQUALS, LESS_THAN,
    LESS_THAN_OR_EQUALS, LIST, LIST_CHECK, LIST_EMPTY, LIST_LEN, MULTIPLICATION, PRINT,
    SUBTRACTION,
};
use evaluator::eval;
use reader::{ParseError, Reader, get_regex, tokenize};
use variable_type::DataType;

use crate::evaluator::Environment;

mod env;
mod evaluator;
mod reader;
mod variable_type;

#[cfg(test)]
mod tests;

fn read(input: String) -> Result<DataType, ParseError> {
    let tokens = tokenize(input, get_regex());
    let mut reader = Reader::new(tokens);
    reader.read()
}

fn print(input: DataType) -> String {
    format!("{:?}", input)
}

fn rep(input: String, repl_env: &mut Environment) -> String {
    let ast = match read(input) {
        Ok(r) => r,
        Err(e) => return e.msg,
    };

    let eval_result = match eval(&ast, repl_env) {
        Ok(r) => r,
        Err(e) => return e.msg,
    };
    let print_result = print(eval_result);
    print_result
}

fn create_repl_env() -> Environment {
    let mut repl_env = Environment::new(None);

    let core_functions = vec![
        ADDITION,
        SUBTRACTION,
        DIVISION,
        MULTIPLICATION,
        PRINT,
        LIST,
        LIST_CHECK,
        LIST_EMPTY,
        LIST_LEN,
        EQUALS,
        GREATER_THAN,
        LESS_THAN,
        LESS_THAN_OR_EQUALS,
        GREATER_THAN_OR_EQUALS,
    ];

    for item in core_functions {
        repl_env.set(item.id.to_string(), DataType::Function(Rc::new(item.func)));
    }

    repl_env
}

fn main() {
    let mut repl_env = create_repl_env();
    rep(
        "(def! not (fn* (a) (if a false true)))".to_string(),
        &mut repl_env,
    );
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
            println!();
            println!("Quitting...");
            break;
        }

        println!("{}", rep(user_input, &mut repl_env))
    }
}
