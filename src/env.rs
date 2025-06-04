use std::cell::RefCell;
use std::rc::Rc;

use crate::evaluator::{Environment, EvalError};
use crate::variable_type::DataType;
use crate::variable_type::DataType::*;

struct CoreFunction {
    pub id: &'static str,
    pub func: fn(&[DataType]) -> Result<DataType, EvalError>,
}

#[allow(unused_assignments)]
pub fn create_repl_env() -> Rc<RefCell<Environment>> {
    let mut repl_env = Environment::new(None);

    macro_rules! set_function {
        ($($l:ident),*) => {
            let mut i = 0;
            $ (
                repl_env.set($l.id.to_string(), DataType::BuiltinFunction((i, &$l.func)));
                i += 1;
            )*
        };
    }

    set_function!(
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
        GREATER_THAN_OR_EQUALS
    );

    Rc::new(RefCell::new(repl_env))
}

const ADDITION: CoreFunction = CoreFunction {
    id: "+",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Integer(num1 + num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Float(num1 + num2)),

                (Some(String(str1)), Some(String(str2))) => Ok(String(str1.to_owned() + str2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for addition!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for addition".to_string(),
            })
        }
    },
};

const MULTIPLICATION: CoreFunction = CoreFunction {
    id: "*",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Integer(num1 * num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Float(num1 * num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for addition!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for addition".to_string(),
            })
        }
    },
};
const SUBTRACTION: CoreFunction = CoreFunction {
    id: "-",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Integer(num1 - num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Float(num1 - num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for subtraction".to_string(),
            })
        }
    },
};

const DIVISION: CoreFunction = CoreFunction {
    id: "/",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Integer(num1 / num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Float(num1 / num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for division".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for division".to_string(),
            })
        }
    },
};

const PRINT: CoreFunction = CoreFunction {
    id: "prn",
    func: |values: &[DataType]| {
        println!(
            "{}",
            values
                .iter()
                .map(|value| format!("{:?}", value))
                .collect::<Vec<_>>()
                .join(" ")
        );

        Ok(DataType::Nil())
    },
};

const LIST: CoreFunction = CoreFunction {
    id: "list",
    func: |values: &[DataType]| {
        let mut children = vec![];
        for value in values {
            children.push(value.clone());
        }
        Ok(DataType::List(children))
    },
};

const LIST_CHECK: CoreFunction = CoreFunction {
    id: "list?",
    func: |values: &[DataType]| match values.first() {
        Some(DataType::List(_)) => Ok(DataType::Bool(true)),
        None => Err(EvalError {
            msg: "No arguments given to list?".to_string(),
        }),
        _ => Ok(DataType::Bool(false)),
    },
};

const LIST_EMPTY: CoreFunction = CoreFunction {
    id: "empty?",
    func: |values: &[DataType]| {
        if let Some(DataType::List(children)) = values.first() {
            if children.len() == 0 {
                Ok(DataType::Bool(true))
            } else {
                Ok(DataType::Bool(false))
            }
        } else {
            Err(EvalError {
                msg: "No arguments given to empty?".to_string(),
            })
        }
    },
};

const LIST_LEN: CoreFunction = CoreFunction {
    id: "count",
    func: |values: &[DataType]| {
        if let Some(DataType::List(children)) = values.first() {
            let length = match children.len().try_into() {
                Ok(l) => l,
                Err(_) => {
                    return Err(EvalError {
                        msg: "List is too long to return length!".to_string(),
                    });
                }
            };
            Ok(DataType::Integer(length))
        } else {
            Err(EvalError {
                msg: "No arguments given to empty?".to_string(),
            })
        }
    },
};

const EQUALS: CoreFunction = CoreFunction {
    id: "=",
    func: |values: &[DataType]| {
        let (Some(var1), Some(var2)) = (values.get(0), values.get(1)) else {
            return Err(EvalError {
                msg: "Not enough arguments passed to =".to_string(),
            });
        };

        Ok(DataType::Bool(*var1 == *var2))
    },
};

// TODO: Write a macro for these
const GREATER_THAN: CoreFunction = CoreFunction {
    id: ">",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Bool(num1 > num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Bool(num1 > num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for subtraction".to_string(),
            })
        }
    },
};

const LESS_THAN: CoreFunction = CoreFunction {
    id: "<",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Bool(num1 < num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Bool(num1 < num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for subtraction".to_string(),
            })
        }
    },
};

const GREATER_THAN_OR_EQUALS: CoreFunction = CoreFunction {
    id: ">=",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Bool(num1 >= num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Bool(num1 >= num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for subtraction".to_string(),
            })
        }
    },
};

const LESS_THAN_OR_EQUALS: CoreFunction = CoreFunction {
    id: "<=",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Bool(num1 <= num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Bool(num1 <= num2)),

                _ => Err(EvalError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for subtraction".to_string(),
            })
        }
    },
};
