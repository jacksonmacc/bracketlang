use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::variable_type::DataType;

#[derive(Debug)]
pub struct EvalError {
    pub msg: String,
}

#[derive(Clone)]
pub struct Environment {
    outer: Option<Rc<RefCell<Self>>>,
    data: HashMap<String, DataType>,
}

impl Environment {
    pub fn new(outer: Option<Rc<RefCell<Self>>>) -> Environment {
        Self {
            outer,
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

                outer_env.borrow_mut().get(sym)
            }
        }
    }
}

pub fn eval<'a>(
    ast: &'a DataType,
    repl_env: Rc<RefCell<Environment>>,
) -> Result<DataType, EvalError> {
    let mut ast = ast;
    let mut repl_env: Rc<RefCell<Environment>> = repl_env;

    loop {
        match ast {
            DataType::List(children) => {
                match children.first() {
                    Some(DataType::Symbol(val)) if *val == "def!".to_string() => {
                        return eval_def(&children[1..], repl_env.clone());
                    }

                    Some(DataType::Symbol(val)) if *val == "let*".to_string() => {
                        match prepare_tail_call_let(&children[1..], repl_env) {
                            Ok((new_ast, new_env)) => {
                                ast = new_ast;
                                repl_env = Rc::new(RefCell::new(new_env));
                                continue;
                            }
                            Err(e) => return Err(e),
                        };
                    }

                    Some(DataType::Symbol(val)) if *val == "do".to_string() => {
                        match prepare_tail_call_do(&children[1..], &repl_env) {
                            Ok(new_ast) => {
                                ast = new_ast;
                                continue;
                            }
                            Err(e) => return Err(e),
                        };
                    }

                    Some(DataType::Symbol(val)) if *val == "if".to_string() => {
                        match prepare_tail_call_if(&children[1..], &repl_env) {
                            Ok(new_ast) => {
                                ast = new_ast;
                                continue;
                            }
                            Err(e) => return Err(e),
                        };
                    }

                    Some(DataType::Symbol(val)) if *val == "fn*".to_string() => {
                        return eval_closure(&children[1..], repl_env.clone());
                    }
                    _ => {}
                };

                let evaluated: Vec<DataType> = children
                    .iter()
                    .map(|child| eval(child, repl_env.clone()))
                    .collect::<Result<_, EvalError>>()?;

                match evaluated.first() {
                    Some(DataType::Function(function)) => return Ok(function(&evaluated[1..])?),
                    Some(DataType::BuiltinFunction(function)) => {
                        return Ok(function.1(&evaluated[1..])?);
                    }
                    // TODO: Check that this isn't supposed to error
                    None | Some(_) => return Ok(DataType::List(evaluated)),
                };
            }

            DataType::Vector(list) => {
                let evaluated: Vec<DataType> = list
                    .iter()
                    .map(|child| eval(child, repl_env.clone()))
                    .collect::<Result<_, EvalError>>()?;

                return Ok(DataType::Vector(evaluated));
            }

            DataType::Dictionary(dict) => {
                let evaluated: HashMap<String, DataType> = dict
                    .iter()
                    .map(|child| match eval(child.1, repl_env.clone()) {
                        Ok(result) => Ok((child.0.clone(), result)),
                        Err(err) => Err(err),
                    })
                    .collect::<Result<HashMap<String, DataType>, EvalError>>()?;

                return Ok(DataType::Dictionary(evaluated));
            }

            DataType::Symbol(sym) => {
                if let Some(val) = repl_env.borrow_mut().get(sym) {
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

fn eval_def(args: &[DataType], env: Rc<RefCell<Environment>>) -> Result<DataType, EvalError> {
    if let (Some(DataType::Symbol(sym)), Some(val)) = (args.get(0), args.get(1)) {
        let evaluated_val = eval(val, env.clone())?;
        env.borrow_mut().set(sym.to_owned(), evaluated_val.clone());

        return Ok(evaluated_val);
    } else {
        return Err(EvalError {
            msg: "Incorrect usage of def!".to_string(),
        });
    }
}

fn prepare_tail_call_let<'a>(
    args: &'a [DataType],
    env: Rc<RefCell<Environment>>,
) -> Result<(&'a DataType, Environment), EvalError> {
    if let (Some(DataType::List(children)), Some(data)) = (args.get(0), args.get(1)) {
        let mut new_env = Environment::new(Some(env.clone()));
        let mut i = 0;

        loop {
            match children.get(i) {
                Some(DataType::Symbol(val1)) => {
                    if let Some(val2) = children.get(i + 1) {
                        new_env.set(val1.to_owned(), val2.clone());
                    } else {
                        return Err(EvalError {
                            msg: "Each symbol in a let* environment should have a value"
                                .to_string(),
                        });
                    };
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
        return Ok((data, new_env));
    } else {
        return Err(EvalError {
            msg: "Incorrect arguments for let*".to_string(),
        });
    }
}

fn prepare_tail_call_do<'a>(
    args: &'a [DataType],
    env: &Rc<RefCell<Environment>>,
) -> Result<&'a DataType, EvalError> {
    for child in &args[..args.len() - 1] {
        let _ = eval(child, env.clone());
    }
    if let Some(final_child) = args.last() {
        Ok(final_child)
    } else {
        return Err(EvalError {
            msg: "No arguments given for do".to_string(),
        });
    }
}

fn prepare_tail_call_if<'a>(
    args: &'a [DataType],
    env: &Rc<RefCell<Environment>>,
) -> Result<&'a DataType, EvalError> {
    let Some(condition) = args.get(0) else {
        return Err(EvalError {
            msg: "No condition for if expression".to_string(),
        });
    };
    match eval(condition, env.clone())? {
        DataType::Bool(false) | DataType::Nil() => {
            if let Some(arg) = args.get(2) {
                return Ok(arg);
            } else {
                return Ok(&DataType::Nil());
            }
        }
        _ => {
            if let Some(arg) = args.get(1) {
                return Ok(arg);
            } else {
                return Err(EvalError {
                    msg: "No body for if expression".to_string(),
                });
            }
        }
    }
}

fn eval_closure(args: &[DataType], env: Rc<RefCell<Environment>>) -> Result<DataType, EvalError> {
    if let Some(DataType::List(params)) = args.get(0) {
        let param_names = params
            .iter()
            .map(|param| {
                if let DataType::Symbol(param_name) = param {
                    Ok(param_name.clone())
                } else {
                    Err(EvalError {
                        msg: format!("{:?} cannot be used as a parameter name", param),
                    })
                }
            })
            .collect::<Result<Vec<String>, EvalError>>()?;

        let closure_env = Environment {
            outer: Some(env.clone()),
            data: HashMap::new(),
        };

        let Some(closure_body_ref) = args.get(1) else {
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
                    let (name, param) = match (param_names.get(i), params.get(i)) {
                        (Some(ampersand), Some(_)) if ampersand == "&" => {
                            let Some(name) = param_names.get(i + 1) else {
                                return Err(EvalError {
                                    msg: "& found in closure without variadic argument name"
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

                            closure_env.set(name.to_owned(), DataType::List(children));

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

                eval(&closure_body, Rc::new(RefCell::new(closure_env.clone())))
            },
        )));
    } else {
        return Err(EvalError {
            msg: "Expected parameter list for function".to_string(),
        });
    }
}
