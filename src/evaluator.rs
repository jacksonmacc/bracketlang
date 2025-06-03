use std::{collections::HashMap, rc::Rc};

use crate::variable_type::DataType;

#[derive(Debug)]
pub struct EvalError {
    pub msg: String,
}

#[derive(Clone)]
pub struct Environment {
    outer: Option<Box<Self>>,
    data: HashMap<String, DataType>,
}

impl Environment {
    pub fn new(outer: Option<Box<Self>>) -> Environment {
        Self {
            outer: outer,
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, sym: String, value: DataType) {
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

// "Why does it have to be all in one function?" dont ask.
pub fn eval<'a>(ast: &'a DataType, repl_env: &mut Environment) -> Result<DataType, EvalError> {
    let mut ast = ast;
    let repl_env = repl_env;

    loop {
        match ast {
            DataType::List(children) => {
                match children.first() {
                    Some(DataType::Symbol(val)) if *val == "def!".to_string() => {
                        return eval_def(&children[1..], repl_env);
                    }

                    Some(DataType::Symbol(val)) if *val == "let*".to_string() => {
                        match prepare_tail_call_let(&children[1..], repl_env, &mut ast) {
                            Some(e) => return Err(e),
                            None => continue,
                        };
                    }

                    Some(DataType::Symbol(val)) if *val == "do".to_string() => {
                        match prepare_tail_call_do(&children[1..], repl_env, &mut ast) {
                            Some(e) => return Err(e),
                            None => continue,
                        };
                    }

                    Some(DataType::Symbol(val)) if *val == "if".to_string() => {
                        let Some(condition) = children.get(1) else {
                            return Err(EvalError {
                                msg: "No condition for if expression".to_string(),
                            });
                        };
                        match eval(condition, repl_env)? {
                            DataType::Bool(false) | DataType::Nil() => {
                                if let Some(arg) = children.get(3) {
                                    ast = arg;
                                    continue;
                                } else {
                                    return Ok(DataType::Nil());
                                }
                            }
                            _ => {
                                if let Some(arg) = children.get(2) {
                                    ast = arg;
                                    continue;
                                } else {
                                    return Err(EvalError {
                                        msg: "No body for if expression".to_string(),
                                    });
                                }
                            }
                        }
                    }

                    Some(DataType::Symbol(val)) if *val == "fn*".to_string() => {
                        if let Some(DataType::List(params)) = children.get(1) {
                            let param_names = params
                                .iter()
                                .map(|param| {
                                    if let DataType::Symbol(param_name) = param {
                                        Ok(param_name.clone())
                                    } else {
                                        Err(EvalError {
                                            msg: format!(
                                                "{:?} cannot be used as a parameter name",
                                                param
                                            ),
                                        })
                                    }
                                })
                                .collect::<Result<Vec<String>, EvalError>>()?;

                            let closure_env = Environment {
                                outer: Some(Box::new(repl_env.clone())),
                                data: HashMap::new(),
                            };

                            let Some(closure_body_ref) = children.get(2) else {
                                return Err(EvalError {
                                    msg: "No body for closure".to_string(),
                                });
                            };

                            let closure_body = closure_body_ref.clone();

                            return Ok(DataType::Function(Rc::new(
                                move |params: &[DataType]| -> Result<DataType, EvalError> {
                                    let mut closure_env = closure_env.clone();
                                    let mut i = 0;

                                    loop {
                                        let (name, param) = match (
                                            param_names.get(i),
                                            params.get(i),
                                        ) {
                                            (Some(ampersand), Some(_)) if ampersand == "&" => {
                                                let Some(name) = param_names.get(i + 1) else {
                                                    return Err(EvalError {
                                                            msg:
                                                                "& found in closure without variadic argument name"
                                                                    .to_string(),
                                                        });
                                                };

                                                let mut children = vec![];
                                                let mut i2 = 0;

                                                loop {
                                                    let Some(param) = params.get(i2) else {
                                                        break;
                                                    };
                                                    children.push(param.clone());
                                                    i2 += 1;
                                                }

                                                closure_env
                                                    .set(name.to_owned(), DataType::List(children));

                                                break;
                                            }

                                            (Some(name), Some(param)) => (name, param),

                                            (None, None) => {
                                                break;
                                            }

                                            _ => {
                                                return Err(EvalError {
                                                    msg: "Parameters given do not match expected parameters"
                                                        .to_string(),
                                                });
                                            }
                                        };

                                        closure_env.set(name.to_owned(), param.clone());
                                        i += 1;
                                    }

                                    eval(&closure_body, &mut closure_env)
                                },
                            )));
                        } else {
                            return Err(EvalError {
                                msg: "Expected parameter list for function".to_string(),
                            });
                        }
                    }
                    _ => {}
                };

                let evaluated: Vec<DataType> = children
                    .iter()
                    .map(|child| eval(child, repl_env))
                    .collect::<Result<_, EvalError>>()?;

                match evaluated.first() {
                    Some(DataType::Function(function)) => return Ok(function(&evaluated[1..])?),
                    // TODO: Check that this isn't supposed to error
                    None | Some(_) => return Ok(DataType::List(evaluated)),
                };
            }

            DataType::Vector(list) => {
                let evaluated: Vec<DataType> = list
                    .iter()
                    .map(|child| eval(child, repl_env))
                    .collect::<Result<_, EvalError>>()?;

                return Ok(DataType::Vector(evaluated));
            }

            DataType::Dictionary(dict) => {
                let evaluated: HashMap<String, DataType> = dict
                    .iter()
                    .map(|child| match eval(child.1, repl_env) {
                        Ok(result) => Ok((child.0.clone(), result)),
                        Err(err) => Err(err),
                    })
                    .collect::<Result<HashMap<String, DataType>, EvalError>>()?;

                return Ok(DataType::Dictionary(evaluated));
            }

            DataType::Symbol(sym) => {
                if let Some(val) = repl_env.get(sym) {
                    return Ok(val);
                } else {
                    return Err(EvalError {
                        msg: format!("Unknown symbol: {}", sym),
                    });
                };
            }

            _ => return Ok(ast.clone()),
        }
    }
}

fn eval_def(args: &[DataType], env: &mut Environment) -> Result<DataType, EvalError> {
    if let (Some(DataType::Symbol(sym)), Some(val)) = (args.get(0), args.get(1)) {
        let evaluated_val = eval(val, env)?;
        env.set(sym.to_owned(), evaluated_val.clone());

        return Ok(evaluated_val);
    } else {
        return Err(EvalError {
            msg: "Incorrect usage of def!".to_string(),
        });
    }
}

fn prepare_tail_call_let<'a>(
    args: &'a [DataType],
    env: &mut Environment,
    ast: &mut &'a DataType,
) -> Option<EvalError> {
    if let (Some(DataType::List(children)), Some(data)) = (args.get(0), args.get(1)) {
        *env = Environment::new(Some(Box::new(env.clone())));
        let mut i = 0;

        loop {
            match children.get(i) {
                Some(DataType::Symbol(val1)) => {
                    if let Some(val2) = children.get(i + 1) {
                        env.set(val1.to_owned(), val2.clone());
                    } else {
                        return Some(EvalError {
                            msg: "Each symbol in a let* environment should have a value"
                                .to_string(),
                        });
                    };
                }

                Some(_) => {
                    return Some(EvalError {
                        msg: "Invalid symbol to set in let*".to_string(),
                    });
                }

                None => {
                    break;
                }
            }

            i += 2;
        }

        *ast = data;
        return None;
    } else {
        return Some(EvalError {
            msg: "Incorrect arguments for let*".to_string(),
        });
    }
}

fn prepare_tail_call_do<'a>(
    args: &'a [DataType],
    env: &mut Environment,
    ast: &mut &'a DataType,
) -> Option<EvalError> {
    for child in &args[..args.len() - 1] {
        let _ = eval(child, env);
    }
    if let Some(final_child) = args.last() {
        *ast = final_child;
        None
    } else {
        return Some(EvalError {
            msg: "No arguments given for do".to_string(),
        });
    }
}
