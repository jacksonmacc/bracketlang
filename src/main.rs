use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
    rc::Rc,
};

use env::{ADDITION, DIVISION, MULTIPLICATION, SUBTRACTION};
use reader::{DataType, ParseError, Reader, get_regex, tokenize};

mod env;
mod reader;

#[cfg(test)]
mod tests;

struct Environment {
    outer: Option<Box<Self>>,
    data: HashMap<String, DataType>,
}

impl Environment {
    fn set(&mut self, sym: String, value: DataType) {
        self.data.insert(sym, value);
    }

    fn get(&self, sym: &String) -> Option<DataType> {
        match self.data.get(sym) {
            Some(v) => Some(v.clone()),
            None => {
                let Some(ref outer_env) = self.outer else {
                    return None;
                };

                outer_env.get(sym)
            }
        }
    }
}

fn read(input: String) -> Result<DataType, ParseError> {
    let tokens = tokenize(input, get_regex());
    let mut reader = Reader::new(tokens);
    reader.read()
}

fn eval(ast: &DataType, repl_env: &Environment) -> Result<DataType, EvalError> {
    match ast {
        DataType::Node(children) => {
            let evaluated: Vec<DataType> = children
                .iter()
                .map(|child| eval(child, repl_env))
                .collect::<Result<_, EvalError>>()?;

            if let Some(DataType::Symbol(sym)) = evaluated.first() {
                let Some(DataType::Function(func)) = repl_env.get(sym) else {
                    return Err(EvalError {
                        msg: format!("Couldn't find symbol \"{}\"", sym),
                    });
                };
                Ok(func(&evaluated[1..])?)
            } else {
                Err(EvalError {
                    msg: "Created function without name!".to_string(),
                })
            }
        }

        DataType::List(list) => {
            let evaluated: Vec<DataType> = list
                .iter()
                .map(|child| eval(child, repl_env))
                .collect::<Result<_, EvalError>>()?;
            Ok(DataType::List(evaluated))
        }

        DataType::Dictionary(dict) => {
            let evaluated: HashMap<String, DataType> = dict
                .iter()
                .map(|child| match eval(child.1, repl_env) {
                    Ok(result) => Ok((child.0.clone(), result)),
                    Err(err) => Err(err),
                })
                .collect::<Result<HashMap<String, DataType>, EvalError>>()?;
            Ok(DataType::Dictionary(evaluated))
        }
        _ => Ok(ast.clone()),
    }
}

fn print(input: DataType) -> String {
    format!("{:?}", input)
}

fn rep(input: String, repl_env: &Environment) -> String {
    let ast = match read(input) {
        Ok(r) => r,
        Err(e) => return e.msg,
    };

    let eval_result = match eval(&ast, &repl_env) {
        Ok(r) => r,
        Err(e) => return e.msg,
    };
    let print_result = print(eval_result);
    print_result
}

#[derive(Debug)]
struct EvalError {
    msg: String,
}

fn create_repl_env() -> Environment {
    let mut repl_env = Environment {
        outer: None,
        data: HashMap::new(),
    };
    repl_env.set(
        ADDITION.id.to_string(),
        DataType::Function(Rc::new(ADDITION.func)),
    );
    repl_env.set(
        SUBTRACTION.id.to_string(),
        DataType::Function(Rc::new(SUBTRACTION.func)),
    );
    repl_env.set(
        DIVISION.id.to_string(),
        DataType::Function(Rc::new(DIVISION.func)),
    );
    repl_env.set(
        MULTIPLICATION.id.to_string(),
        DataType::Function(Rc::new(MULTIPLICATION.func)),
    );

    repl_env
}

fn main() {
    let repl_env = create_repl_env();
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

        println!("{}", rep(user_input, &repl_env))
    }
}
