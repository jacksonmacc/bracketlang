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

#[derive(Clone)]
struct Environment<'a> {
    outer: Option<Box<&'a Self>>,
    data: HashMap<String, DataType>,
}

impl Environment<'_> {
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

fn eval(ast: &DataType, repl_env: &mut Environment) -> Result<DataType, EvalError> {
    match ast {
        DataType::List(children) => {
            match children.first() {
                Some(DataType::Symbol(val)) if *val == "def!".to_string() => {
                    if let (Some(DataType::Symbol(sym)), Some(val)) =
                        (children.get(1), children.get(2))
                    {
                        let evaluated_val = eval(val, repl_env)?;
                        repl_env.set(sym.to_owned(), evaluated_val.clone());
                        return Ok(evaluated_val);
                    } else {
                        return Err(EvalError {
                            msg: "Incorrect usage of def!".to_string(),
                        });
                    }
                }
                Some(DataType::Symbol(val)) if *val == "let*".to_string() => {
                    let mut new_env = Environment {
                        outer: Some(Box::new(repl_env)),
                        data: HashMap::new(),
                    };

                    if let (Some(DataType::List(children)), Some(data)) =
                        (children.get(1), children.get(2))
                    {
                        let mut i = 0;
                        loop {
                            match children.get(i) {
                                Some(DataType::Symbol(val1)) => {
                                    if let Some(val2) = children.get(i + 1) {
                                        new_env.set(val1.to_owned(), val2.clone());
                                    } else {
                                        return Err(EvalError {
                                            msg:
                                                "Each symbol in a let* environment should have a value"
                                                    .to_string(),
                                        });
                                    }
                                }
                                Some(_) => {
                                    return Err(EvalError {
                                        msg: "Invalid symbol to set in let*".to_string(),
                                    });
                                }
                                None => {
                                    break;
                                }
                            }
                            i += 2;
                        }

                        return Ok(eval(data, &mut new_env)?);
                    } else {
                        return Err(EvalError {
                            msg: "Incorrect arguments for let*".to_string(),
                        });
                    }
                }
                Some(DataType::Symbol(val)) if *val == "do".to_string() => {
                    for child in &children[1..children.len() - 2] {
                        let _ = eval(child, repl_env);
                    }
                    if let Some(final_child) = children.last() {
                        return eval(final_child, repl_env);
                    } else {
                        return Err(EvalError {
                            msg: "No arguments given for do".to_string(),
                        });
                    }
                }
                Some(DataType::Symbol(val)) if *val == "if".to_string() => match children.get(1) {
                    Some(DataType::Bool(false) | DataType::Nil()) => {
                        if let Some(arg) = children.get(3) {
                            return eval(arg, repl_env);
                        } else {
                            return Ok(DataType::Nil());
                        }
                    }
                    Some(_) => {
                        if let Some(arg) = children.get(2) {
                            return eval(arg, repl_env);
                        } else {
                            return Err(EvalError {
                                msg: "No body for if expression".to_string(),
                            });
                        }
                    }
                    None => {
                        return Err(EvalError {
                            msg: "No condition for if expression".to_string(),
                        });
                    }
                },
                _ => {}
            };

            let evaluated: Vec<DataType> = children
                .iter()
                .map(|child| eval(child, repl_env))
                .collect::<Result<_, EvalError>>()?;

            match evaluated.first() {
                Some(DataType::Function(function)) => Ok(function(&evaluated[1..])?),
                None | Some(_) => Err(EvalError {
                    msg: "List has no leading function".to_string(),
                }),
            }
        }

        DataType::Vector(list) => {
            let evaluated: Vec<DataType> = list
                .iter()
                .map(|child| eval(child, repl_env))
                .collect::<Result<_, EvalError>>()?;
            Ok(DataType::Vector(Rc::new(evaluated)))
        }

        DataType::Dictionary(dict) => {
            let evaluated: HashMap<String, DataType> = dict
                .iter()
                .map(|child| match eval(child.1, repl_env) {
                    Ok(result) => Ok((child.0.clone(), result)),
                    Err(err) => Err(err),
                })
                .collect::<Result<HashMap<String, DataType>, EvalError>>()?;
            Ok(DataType::Dictionary(Rc::new(evaluated)))
        }
        DataType::Symbol(sym) => {
            if let Some(val) = repl_env.get(sym) {
                Ok(val)
            } else {
                Err(EvalError {
                    msg: format!("Unknown symbol: {}", sym),
                })
            }
        }
        _ => Ok(ast.clone()),
    }
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

#[derive(Debug)]
struct EvalError {
    msg: String,
}

fn create_repl_env() -> Environment<'static> {
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
    let mut repl_env = create_repl_env();
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
