use std::{
    cell::RefCell,
    io::{Write, stdin, stdout},
    rc::Rc,
};

use evaluator::eval;
use reader::{ParseError, Reader, get_regex, tokenize};
use variable_type::DataType;

use crate::{env::create_repl_env, evaluator::Environment};

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

fn rep(input: String, repl_env: Rc<RefCell<Environment>>) -> String {
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

fn main() {
    let repl_env = create_repl_env();
    rep(
        "(def! not (fn* (a) (if a false true)))".to_string(),
        repl_env.clone(),
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

        println!("{}", rep(user_input, repl_env.clone()))
    }
}
