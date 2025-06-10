use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use crate::evaluator::{Environment, RuntimeError};
use crate::read;
use crate::variable_type::DataType;
use crate::variable_type::DataType::*;

pub struct CoreFunction {
    pub id: &'static str,
    pub func: fn(&[DataType]) -> Result<DataType, RuntimeError>,
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
        CONCAT,
        NTH,
        FIRST,
        REST,
        THROW,
        APPLY,
        MAP,
        CHECK_NIL,
        CHECK_TRUE,
        CHECK_FALSE,
        CHECK_SYMBOL
    );

    Rc::new(RefCell::new(repl_env))
}

macro_rules! type_check {
    ($a:pat) => {
        |values: &[DataType]| match values.first() {
            Some($a) => Ok(DataType::Bool(true)),
            None => Err(RuntimeError {
                msg: "No arguments given to data type check".to_string(),
            }),
            _ => Ok(DataType::Bool(false)),
        }
    };
}
const ADDITION: CoreFunction = CoreFunction {
    id: "+",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Integer(num1 + num2)),

                (Some(Float(num1)), Some(Float(num2))) => Ok(Float(num1 + num2)),

                (Some(String(str1)), Some(String(str2))) => Ok(String(str1.to_owned() + str2)),

                _ => Err(RuntimeError {
                    msg: "Incorrect types for addition!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for addition!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for division".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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
    func: type_check!(DataType::List(_)),
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
            Err(RuntimeError {
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
                    return Err(RuntimeError {
                        msg: "List is too long to return length!".to_string(),
                    });
                }
            };
            Ok(DataType::Integer(length))
        } else {
            Err(RuntimeError {
                msg: "No arguments given to empty?".to_string(),
            })
        }
    },
};

const EQUALS: CoreFunction = CoreFunction {
    id: "=",
    func: |values: &[DataType]| {
        let (Some(var1), Some(var2)) = (values.get(0), values.get(1)) else {
            return Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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

                _ => Err(RuntimeError {
                    msg: "Incorrect types for subtraction!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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
                    Err(e) => Err(RuntimeError { msg: e.msg }),
                },

                _ => Err(RuntimeError {
                    msg: "Incorrect arguments for read-str!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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
                    Err(e) => Err(RuntimeError {
                        msg: format!("Couldn't load file: {}", e.to_string()),
                    }),
                },

                _ => Err(RuntimeError {
                    msg: "Incorrect arguments for slurp!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
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
                return Err(RuntimeError {
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
            return Err(RuntimeError {
                msg: "Not enough arguments to atom".to_string(),
            });
        };

        Ok(DataType::Atom(Rc::new(RefCell::new(val.clone()))))
    },
};

const CHECK_ATOM: CoreFunction = CoreFunction {
    id: "atom?",
    func: type_check!(DataType::Atom(_)),
};

pub const DEREF: CoreFunction = CoreFunction {
    id: "deref",
    func: |values: &[DataType]| {
        let Some(Atom(atom)) = values.first() else {
            return Err(RuntimeError {
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
            return Err(RuntimeError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let Some(val) = values.get(1) else {
            return Err(RuntimeError {
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
            return Err(RuntimeError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let Some(Closure(func)) = values.get(1) else {
            return Err(RuntimeError {
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
            return Err(RuntimeError {
                msg: "Incorrect arguments to deref".to_string(),
            });
        };

        let Some(DataType::List(list) | DataType::Vector(list)) = values.get(1) else {
            return Err(RuntimeError {
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
                return Err(RuntimeError {
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

const NTH: CoreFunction = CoreFunction {
    id: "nth",
    func: |values: &[DataType]| {
        let (Some(List(list) | Vector(list)), Some(Integer(idx))) = (values.get(0), values.get(1))
        else {
            return Err(RuntimeError {
                msg: "Wrong arguments for nth".to_string(),
            });
        };

        match list.get(*idx as usize) {
            Some(v) => Ok(v.clone()),
            None => {
                return Err(RuntimeError {
                    msg: "Index out of bounds".to_string(),
                });
            }
        }
    },
};

const FIRST: CoreFunction = CoreFunction {
    id: "first",
    func: |values: &[DataType]| {
        let Some(List(list) | Vector(list)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Wrong arguments for nth".to_string(),
            });
        };

        match list.first() {
            Some(v) => Ok(v.clone()),
            None => {
                return Err(RuntimeError {
                    msg: "Index out of bounds".to_string(),
                });
            }
        }
    },
};

const REST: CoreFunction = CoreFunction {
    id: "rest",
    func: |values: &[DataType]| {
        let Some(List(list) | Vector(list)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Wrong arguments for nth".to_string(),
            });
        };

        return Ok(DataType::List(list[1..].iter().cloned().collect()));
    },
};

const THROW: CoreFunction = CoreFunction {
    id: "throw",
    func: |values: &[DataType]| {
        let Some(String(string)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Wrong arguments for throw".to_string(),
            });
        };

        return Err(RuntimeError {
            msg: string.to_string(),
        });
    },
};

const APPLY: CoreFunction = CoreFunction {
    id: "apply",
    func: |values: &[DataType]| {
        if let Some(Closure(closure)) = values.get(0) {
            let mut args = vec![];
            for val in &values[1..] {
                match val {
                    List(data_types) | Vector(data_types) => {
                        for val in data_types {
                            args.push(val.clone());
                        }
                    }
                    _ => args.push(val.clone()),
                }
            }
            closure.func(&args)
        } else if let Some(NativeFunction(func)) = values.get(0) {
            let mut args = vec![];
            for val in &values[1..] {
                match val {
                    List(data_types) | Vector(data_types) => {
                        for val in data_types {
                            args.push(val.clone());
                        }
                    }
                    _ => args.push(val.clone()),
                }
            }
            func.1(&args)
        } else {
            return Err(RuntimeError {
                msg: "Wrong arguments for apply".to_string(),
            });
        }
    },
};

const MAP: CoreFunction = CoreFunction {
    id: "map",
    func: |values: &[DataType]| {
        if let (Some(Closure(closure)), Some(List(list) | Vector(list))) =
            (values.get(0), values.get(0))
        {
            let mut result = vec![];
            for val in list {
                result.push(closure.func(&[val.clone()])?);
            }
            Ok(DataType::List(result))
        } else if let (Some(NativeFunction(closure)), Some(List(list) | Vector(list))) =
            (values.get(0), values.get(0))
        {
            let mut result = vec![];
            for val in list {
                result.push(closure.1(&[val.clone()])?);
            }
            Ok(DataType::List(result))
        } else {
            return Err(RuntimeError {
                msg: "Wrong arguments for apply".to_string(),
            });
        }
    },
};

const CHECK_NIL: CoreFunction = CoreFunction {
    id: "nil?",
    func: type_check!(DataType::Nil()),
};

const CHECK_TRUE: CoreFunction = CoreFunction {
    id: "true?",
    func: type_check!(DataType::Bool(true)),
};

const CHECK_FALSE: CoreFunction = CoreFunction {
    id: "false?",
    func: type_check!(DataType::Bool(false)),
};

const CHECK_SYMBOL: CoreFunction = CoreFunction {
    id: "symbol?",
    func: type_check!(DataType::Symbol(_)),
};
