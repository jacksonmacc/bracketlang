use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use crate::evaluator::{Environment, EvalError};
use crate::read;
use crate::variable_type::DataType;
use crate::variable_type::DataType::*;

pub struct CoreFunction {
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
                repl_env.set($l.id.to_string(), DataType::NativeFunction((i, &$l.func)));
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
        CHECK_LIST,
        LIST_EMPTY,
        LIST_LEN,
        EQUALS,
        GREATER_THAN,
        LESS_THAN,
        LESS_THAN_OR_EQUALS,
        GREATER_THAN_OR_EQUALS,
        READ_STR,
        SLURP,
        STR,
        ATOM,
        CHECK_ATOM,
        DEREF,
        RESET_ATOM,
        SWAP_ATOM,
        CONS,
        CONCAT
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

const CHECK_LIST: CoreFunction = CoreFunction {
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

const READ_STR: CoreFunction = CoreFunction {
    id: "read-string",
    func: |values: &[DataType]| {
        if values.len() == 1 {
            match values.get(0) {
                Some(String(str)) => match read(str.to_string()) {
                    Ok(val) => Ok(val),
                    Err(e) => Err(EvalError { msg: e.msg }),
                },

                _ => Err(EvalError {
                    msg: "Incorrect arguments for read-str!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for read-str".to_string(),
            })
        }
    },
};

const SLURP: CoreFunction = CoreFunction {
    id: "slurp",
    func: |values: &[DataType]| {
        if values.len() == 1 {
            match values.get(0) {
                Some(String(path)) => match fs::read_to_string(path) {
                    Ok(file) => Ok(DataType::String(file)),
                    Err(e) => Err(EvalError {
                        msg: format!("Couldn't load file: {}", e.to_string()),
                    }),
                },

                _ => Err(EvalError {
                    msg: "Incorrect arguments for slurp!".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Incorrect number of arguments for slurp".to_string(),
            })
        }
    },
};

const STR: CoreFunction = CoreFunction {
    id: "str",
    func: |values: &[DataType]| {
        let mut end_str = "".to_string();
        for value in values {
            let DataType::String(str) = value else {
                return Err(EvalError {
                    msg: "All arguments to str should be strings".to_string(),
                });
            };

            end_str += str;
        }

        Ok(DataType::String(end_str))
    },
};

const ATOM: CoreFunction = CoreFunction {
    id: "atom",
    func: |values: &[DataType]| {
        let Some(val) = values.first() else {
            return Err(EvalError {
                msg: "Not enough arguments to atom".to_string(),
            });
        };

        Ok(DataType::Atom(Rc::new(RefCell::new(val.clone()))))
    },
};

const CHECK_ATOM: CoreFunction = CoreFunction {
    id: "atom?",
    func: |values: &[DataType]| {
        let Some(_) = values.first() else {
            return Err(EvalError {
                msg: "Not enough arguments to atom?".to_string(),
            });
        };

        if let Some(Atom(_)) = values.first() {
            Ok(Bool(true))
        } else {
            Ok(Bool(false))
        }
    },
};

pub const DEREF: CoreFunction = CoreFunction {
    id: "deref",
    func: |values: &[DataType]| {
        let Some(Atom(atom)) = values.first() else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        Ok((**atom).borrow().clone())
    },
};

const RESET_ATOM: CoreFunction = CoreFunction {
    id: "reset!",
    func: |values: &[DataType]| {
        let Some(Atom(atom)) = values.first() else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let Some(val) = values.get(1) else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        atom.replace(val.clone());

        Ok(val.clone())
    },
};

const SWAP_ATOM: CoreFunction = CoreFunction {
    id: "swap!",
    func: |values: &[DataType]| {
        let Some(Atom(atom_value)) = values.first() else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let Some(Closure(func)) = values.get(1) else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let mut args: Vec<DataType> = vec![];
        args.push(atom_value.borrow().clone());
        for value in &values[2..] {
            args.push(value.clone());
        }

        atom_value.replace(func.func(&args[0..])?);

        Ok(atom_value.borrow().clone())
    },
};

const CONS: CoreFunction = CoreFunction {
    id: "cons",
    func: |values: &[DataType]| {
        let Some(value) = values.first() else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let Some(DataType::List(list) | DataType::Vector(list)) = values.get(1) else {
            return Err(EvalError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let mut new_list = vec![];
        new_list.push(value.clone());
        new_list.extend(list.iter().cloned());

        Ok(DataType::List(new_list))
    },
};

const CONCAT: CoreFunction = CoreFunction {
    id: "concat",
    func: |values: &[DataType]| {
        let mut result = vec![];
        for list in values {
            let (DataType::List(list) | DataType::Vector(list)) = list else {
                return Err(EvalError {
                    msg: "Incorrect arguments to concat".to_string(),
                });
            };

            for value in list {
                result.push(value.clone());
            }
        }

        Ok(DataType::List(result))
    },
};
