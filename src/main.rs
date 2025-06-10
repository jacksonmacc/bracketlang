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

fn rep(input: String, repl_env: Rc<RefCell<Environment>>) -> Option<String> {
    let ast = match read(input) {
        Ok(r) => r,
        Err(e) => return Some(e.msg),
    };

    let eval_result = match eval(&ast, repl_env.clone(), repl_env.clone()) {
        Ok(r) => r,
        Err(e) => return Some(e.msg),
    };

    if let DataType::Nil() = eval_result {
        None
    } else {
        let print_result = print(eval_result);
        Some(print_result)
    }
}

fn re(input: String, repl_env: Rc<RefCell<Environment>>) -> Result<DataType, String> {
    let ast = match read(input) {
        Ok(r) => r,
        Err(e) => return Err(e.msg),
    };

    let result = match eval(&ast, repl_env.clone(), repl_env.clone()) {
        Ok(r) => r,
        Err(e) => return Err(e.msg),
    };

    Ok(result)
}

fn main() {
    let repl_env = create_repl_env();
    match re(
        "(def! not (fn* (a) (if a false true)))".to_string(),
        repl_env.clone(),
    ) {
        Err(e) => {
            println!("Error in core function definition! {}", e);
            return;
        }
        _ => (),
    };
    match re(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\\nnil)\")))))"
            .to_string(),
        repl_env.clone(),
    ) {
        Err(e) => {
            println!("Error in core function definition! {}", e);
            return;
        }
        _ => (),
    };

    let args: Vec<String> = std::env::args().collect();
    if let Some(filename) = args.get(1) {
        let mut repl_args = vec![];
        for arg in &args[2..] {
            repl_args.push(DataType::String(arg.clone()));
        }
        repl_env
            .borrow_mut()
            .set("*ARGV*".to_string(), DataType::List(repl_args));
        match rep(format!("(load-file \"{}\")", filename), repl_env.clone()) {
            Some(res) => {
                println!("Error: {}", res);
                return;
            }
            None => return,
        }
    }

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

        if let Some(result) = rep(user_input, repl_env.clone()) {
            println!("{}", result);
        }
    }
}
