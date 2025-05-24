use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use reader::{DataType, DataTypeHashable, ParseError, Reader, get_regex, tokenize};

mod reader;

#[cfg(test)]
mod tests;

fn read(input: String) -> Result<DataType, ParseError> {
    let tokens = tokenize(input, get_regex());
    let mut reader = Reader::new(tokens);
    reader.read()
}

fn eval(
    ast: &DataType,
    repl_env: &HashMap<String, impl Fn(&[DataType]) -> Result<DataType, EvalError>>,
) -> Result<DataType, EvalError> {
    match ast {
        DataType::Node(children) => {
            let evaluated: Vec<DataType> = children
                .iter()
                .map(|child| eval(child, repl_env))
                .collect::<Result<_, EvalError>>()?;

            if let Some(DataType::Symbol(sym)) = evaluated.first() {
                let Some(func) = repl_env.get(sym) else {
                    return Err(EvalError {
                        msg: format!("Couldn't find symbol \"{}\"", sym),
                    });
                };
                func(&evaluated[1..])
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
            let evaluated: HashMap<DataTypeHashable, DataType> = dict
                .iter()
                .map(|child| match eval(child.1, repl_env) {
                    Ok(result) => Ok((child.0.clone(), result)),
                    Err(err) => Err(err),
                })
                .collect::<Result<HashMap<DataTypeHashable, DataType>, EvalError>>()?;
            Ok(DataType::Dictionary(evaluated))
        }
        _ => Ok(ast.clone()),
    }
}

fn print(input: DataType) -> String {
    format!("{:?}", input)
}

fn rep(
    input: String,
    repl_env: &HashMap<String, impl Fn(&[DataType]) -> Result<DataType, EvalError>>,
) -> String {
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

fn create_repl_env() -> HashMap<String, impl Fn(&[DataType]) -> Result<DataType, EvalError>> {
    let mut repl_env: HashMap<String, Box<dyn Fn(&[DataType]) -> Result<DataType, EvalError>>> =
        HashMap::new();
    repl_env.insert(
        "+".to_string(),
        Box::new(|values: &[DataType]| {
            if values.len() > 1 {
                match values.first() {
                    Some(DataType::Hashable(DataTypeHashable::Number(_))) => {
                        let mut total = 0;
                        for value in values {
                            match value {
                                DataType::Hashable(DataTypeHashable::Number(num)) => total += num,
                                _ => {
                                    return Err(EvalError {
                                        msg: "Invalid types for addition!".to_string(),
                                    });
                                }
                            }
                        }

                        Ok(DataType::Hashable(DataTypeHashable::Number(total)))
                    }
                    Some(DataType::Hashable(DataTypeHashable::String(_))) => {
                        let mut total = String::new();
                        for value in values {
                            match value {
                                DataType::Hashable(DataTypeHashable::String(string)) => total += string,
                                _ => {
                                    return Err(EvalError {
                                        msg: "Invalid types for addition!".to_string(),
                                    });
                                }
                            }
                        }

                        Ok(DataType::Hashable(DataTypeHashable::String(total)))
                    }
                    Some(DataType::Float(_)) => {
                        let mut total = 0.0;
                        for value in values {
                            match value {
                                DataType::Float(float) => {
                                    total += float
                                }
                                _ => {
                                    return Err(EvalError {
                                        msg: "Invalid types for addition!".to_string(),
                                    });
                                }
                            }
                        }

                        Ok(DataType::Float(total))
                    }
                    None | Some(_) => Err(EvalError {
                        msg: "Invalid types for addition".to_string(),
                    }),
                }
            } else {
                Err(EvalError {
                    msg: "Not enough arguments for \"+\"".to_string(),
                })
            }
        }),
    );
    repl_env.insert(
        "-".to_string(),
        Box::new(|values: &[DataType]| {
            if values.len() == 2 {
                if let (
                    Some(DataType::Hashable(DataTypeHashable::Number(num1))),
                    Some(DataType::Hashable(DataTypeHashable::Number(num2))),
                ) = (values.get(0), values.get(1))
                {
                    Ok(DataType::Hashable(DataTypeHashable::Number(num1 - num2)))
                } else {
                    Err(EvalError {
                        msg: "Incorrect types for subtraction!".to_string(),
                    })
                }
            } else {
                Err(EvalError {
                    msg: "Not enough arguments for \"+\"".to_string(),
                })
            }
        }),
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
            break;
        }

        println!("{}", rep(user_input, &repl_env))
    }
}
