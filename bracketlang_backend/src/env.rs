use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use crate::evaluator::RuntimeError;

#[cfg(not(target_arch = "wasm32"))]
use crate::read;
#[cfg(target_arch = "wasm32")]
use crate::{js_print, prompt, read};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

use crate::variable_type::DataType;
use crate::variable_type::DataType::*;

pub struct CoreFunction {
    pub id: &'static str,
    pub func: fn(&[DataType]) -> Result<DataType, RuntimeError>,
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
pub const ADDITION: CoreFunction = CoreFunction {
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

pub const MODULO: CoreFunction = CoreFunction {
    id: "%",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(num1)), Some(Integer(num2))) => Ok(Integer(num1 % num2)),

                _ => Err(RuntimeError {
                    msg: "Incorrect types for modulo!".to_string(),
                }),
            }
        } else {
            Err(RuntimeError {
                msg: "Incorrect number of arguments for modulo".to_string(),
            })
        }
    },
};

pub const MULTIPLICATION: CoreFunction = CoreFunction {
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

pub const SUBTRACTION: CoreFunction = CoreFunction {
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

pub const DIVISION: CoreFunction = CoreFunction {
    id: "/",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Integer(_)), Some(Integer(0))) => Err(RuntimeError {
                    msg: "Divide by zero error!".to_string(),
                }),

                (Some(Float(_)), Some(Float(0.0))) => Err(RuntimeError {
                    msg: "Divide by zero error!".to_string(),
                }),

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

#[cfg(not(target_arch = "wasm32"))]
pub const PRINT: CoreFunction = CoreFunction {
    id: "prn",
    func: |values: &[DataType]| {
        println!(
            "{}",
            values
                .iter()
                .map(|value| match value {
                    DataType::String(string) => format!("{}", string),
                    _ => format!("{:?}", value),
                })
                .collect::<Vec<_>>()
                .join(" ")
        );

        Ok(DataType::Nil())
    },
};

#[cfg(target_arch = "wasm32")]
pub const PRINT: CoreFunction = CoreFunction {
    id: "prn",
    func: |values: &[DataType]| {
        js_print(&format!(
            "{}",
            values
                .iter()
                .map(|value| match value {
                    DataType::String(string) => format!("{}", string),
                    _ => format!("{:?}", value),
                })
                .collect::<Vec<_>>()
                .join(" ")
        ));

        Ok(DataType::Nil())
    },
};

pub const LIST: CoreFunction = CoreFunction {
    id: "list",
    func: |values: &[DataType]| {
        let mut children = vec![];
        for value in values {
            children.push(value.clone());
        }
        Ok(DataType::List(children))
    },
};

pub const VECTOR: CoreFunction = CoreFunction {
    id: "vector",
    func: |values: &[DataType]| {
        let mut children = vec![];
        for value in values {
            children.push(value.clone());
        }
        Ok(DataType::Vector(children))
    },
};

pub const CHECK_LIST: CoreFunction = CoreFunction {
    id: "list?",
    func: type_check!(DataType::List(_)),
};

pub const LIST_EMPTY: CoreFunction = CoreFunction {
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

pub const LIST_LEN: CoreFunction = CoreFunction {
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

pub const EQUALS: CoreFunction = CoreFunction {
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
pub const GREATER_THAN: CoreFunction = CoreFunction {
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

pub const LESS_THAN: CoreFunction = CoreFunction {
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

pub const GREATER_THAN_OR_EQUALS: CoreFunction = CoreFunction {
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

pub const LESS_THAN_OR_EQUALS: CoreFunction = CoreFunction {
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

pub const READ_STR: CoreFunction = CoreFunction {
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

pub const SLURP: CoreFunction = CoreFunction {
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

pub const STR: CoreFunction = CoreFunction {
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

pub const ATOM: CoreFunction = CoreFunction {
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

pub const CHECK_ATOM: CoreFunction = CoreFunction {
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

pub const RESET_ATOM: CoreFunction = CoreFunction {
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

pub const SWAP_ATOM: CoreFunction = CoreFunction {
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

pub const CONS: CoreFunction = CoreFunction {
    id: "cons",
    func: |values: &[DataType]| {
        let Some(value) = values.first() else {
            return Err(RuntimeError {
                msg: "Incorrect arguments to cons".to_string(),
            });
        };

        let Some(DataType::List(list) | DataType::Vector(list)) = values.get(1) else {
            return Err(RuntimeError {
                msg: "Incorrect arguments to cons".to_string(),
            });
        };

        let mut new_list = vec![];
        new_list.push(value.clone());
        new_list.extend(list.iter().cloned());

        Ok(DataType::List(new_list))
    },
};

pub const CONCAT: CoreFunction = CoreFunction {
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

pub const NTH: CoreFunction = CoreFunction {
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

pub const FIRST: CoreFunction = CoreFunction {
    id: "first",
    func: |values: &[DataType]| {
        let Some(List(list) | Vector(list)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Wrong arguments for first".to_string(),
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

pub const REST: CoreFunction = CoreFunction {
    id: "rest",
    func: |values: &[DataType]| {
        let Some(List(list) | Vector(list)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Wrong arguments for rest".to_string(),
            });
        };

        return Ok(DataType::List(list[1..].iter().cloned().collect()));
    },
};

pub const THROW: CoreFunction = CoreFunction {
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

pub const APPLY: CoreFunction = CoreFunction {
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

pub const MAP: CoreFunction = CoreFunction {
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

pub const CHECK_NIL: CoreFunction = CoreFunction {
    id: "nil?",
    func: type_check!(DataType::Nil()),
};

pub const CHECK_TRUE: CoreFunction = CoreFunction {
    id: "true?",
    func: type_check!(DataType::Bool(true)),
};

pub const CHECK_FALSE: CoreFunction = CoreFunction {
    id: "false?",
    func: type_check!(DataType::Bool(false)),
};

pub const CHECK_SYMBOL: CoreFunction = CoreFunction {
    id: "symbol?",
    func: type_check!(DataType::Symbol(_)),
};

pub const CHECK_VECTOR: CoreFunction = CoreFunction {
    id: "vector?",
    func: type_check!(DataType::Vector(_)),
};

pub const CHECK_SEQUENTIAL: CoreFunction = CoreFunction {
    id: "sequential?",
    func: type_check!(DataType::Vector(_) | DataType::List(_)),
};

pub const CHECK_DICTIONARY: CoreFunction = CoreFunction {
    id: "dict?",
    func: type_check!(DataType::Dictionary(_)),
};

pub const CHECK_STR: CoreFunction = CoreFunction {
    id: "string?",
    func: type_check!(DataType::String(_)),
};

pub const CHECK_INTEGER: CoreFunction = CoreFunction {
    id: "int?",
    func: type_check!(DataType::Integer(_)),
};

pub const CHECK_FLOAT: CoreFunction = CoreFunction {
    id: "float?",
    func: type_check!(DataType::Float(_)),
};

pub const CHECK_FN: CoreFunction = CoreFunction {
    id: "func?",
    func: type_check!(DataType::Closure(_) | DataType::NativeFunction(_)),
};

pub const CHECK_MACRO: CoreFunction = CoreFunction {
    id: "macro?",
    func: |values: &[DataType]| match values.first() {
        Some(DataType::Closure(closure)) if closure.is_macro => Ok(DataType::Bool(true)),
        None => Err(RuntimeError {
            msg: "No arguments given to data type check".to_string(),
        }),
        _ => Ok(DataType::Bool(false)),
    },
};

pub const SYMBOL: CoreFunction = CoreFunction {
    id: "symbol",
    func: |values: &[DataType]| {
        let Some(String(val)) = values.first() else {
            return Err(RuntimeError {
                msg: "Not enough arguments to symbol".to_string(),
            });
        };

        Ok(DataType::Symbol(val.clone()))
    },
};

pub const DICTIONARY: CoreFunction = CoreFunction {
    id: "dict",
    func: |values: &[DataType]| {
        let mut i = 0;
        let mut result = HashMap::new();

        loop {
            let Some(key) = values.get(i) else {
                break;
            };
            let Some(value) = values.get(i + 1) else {
                return Err(RuntimeError {
                    msg: "No value to match key in dict".to_string(),
                });
            };
            result.insert(format!("{:?}", key), value.clone());
            i += 2;
        }

        Ok(Dictionary(result))
    },
};

pub const ASSOC: CoreFunction = CoreFunction {
    id: "assoc",
    func: |values: &[DataType]| {
        let Some(Dictionary(dict)) = values.first() else {
            return Err(RuntimeError {
                msg: "Incorrect arguments for assoc".to_string(),
            });
        };
        let mut i = 0;
        let mut result = dict.clone();

        loop {
            let Some(key) = values.get(i) else {
                break;
            };
            let Some(value) = values.get(i + 1) else {
                return Err(RuntimeError {
                    msg: "No value to match key in assoc".to_string(),
                });
            };
            result.insert(format!("{:?}", key), value.clone());
            i += 2;
        }

        Ok(Dictionary(result))
    },
};

pub const DISSOC: CoreFunction = CoreFunction {
    id: "dissoc",
    func: |values: &[DataType]| {
        let Some(Dictionary(dict)) = values.first() else {
            return Err(RuntimeError {
                msg: "Incorrect arguments for dissoc".to_string(),
            });
        };
        let mut i = 0;
        let mut result = dict.clone();

        loop {
            let Some(key) = values.get(i) else {
                break;
            };

            result.remove(&format!("{:?}", key));
            i += 1;
        }

        Ok(Dictionary(result))
    },
};

pub const GET: CoreFunction = CoreFunction {
    id: "get",
    func: |values: &[DataType]| {
        let (Some(Dictionary(dict)), Some(key)) = (values.get(0), values.get(1)) else {
            return Err(RuntimeError {
                msg: "Incorrect arguments for get".to_string(),
            });
        };
        match dict.get(&format!("{:?}", key)) {
            Some(val) => Ok(val.clone()),
            None => Err(RuntimeError {
                msg: "Key not found in dict".to_string(),
            }),
        }
    },
};

pub const CONTAINS: CoreFunction = CoreFunction {
    id: "contains",
    func: |values: &[DataType]| {
        let (Some(Dictionary(dict)), Some(key)) = (values.get(0), values.get(1)) else {
            return Err(RuntimeError {
                msg: "Incorrect arguments for contains".to_string(),
            });
        };
        Ok(Bool(dict.contains_key(&format!("{:?}", key))))
    },
};

pub const KEYS: CoreFunction = CoreFunction {
    id: "keys",
    func: |values: &[DataType]| {
        let Some(Dictionary(dict)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Incorrect arguments for keys".to_string(),
            });
        };
        Ok(List(dict.keys().cloned().map(|val| String(val)).collect()))
    },
};

pub const VALUES: CoreFunction = CoreFunction {
    id: "values",
    func: |values: &[DataType]| {
        let Some(Dictionary(dict)) = values.get(0) else {
            return Err(RuntimeError {
                msg: "Incorrect arguments for values".to_string(),
            });
        };
        Ok(List(dict.values().cloned().collect()))
    },
};

#[cfg(not(target_arch = "wasm32"))]
pub const TIME_MS: CoreFunction = CoreFunction {
    id: "time-ms",
    func: |_values: &[DataType]| {
        Ok(Integer(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .try_into()
                .unwrap(),
        ))
    },
};

#[cfg(target_arch = "wasm32")]
pub const TIME_MS: CoreFunction = CoreFunction {
    id: "time-ms",
    func: |_values: &[DataType]| {
        use crate::js_get_time;

        Ok(Integer(js_get_time().into()))
    },
};

#[cfg(not(target_arch = "wasm32"))]
pub const INPUT: CoreFunction = CoreFunction {
    id: "input",
    func: |values: &[DataType]| {
        if let Some(DataType::String(string)) = values.get(0) {
            use std::io::{Write, stdin, stdout};

            print!("{}", string);
            stdout()
                .flush()
                .expect("Flushing stdout should have worked.");
            let mut user_input = std::string::String::new();

            stdin()
                .read_line(&mut user_input)
                .expect("Didn't enter a correct string");
            Ok(DataType::String(user_input))
        } else {
            use std::io::{Write, stdin, stdout};

            stdout()
                .flush()
                .expect("Flushing stdout should have worked.");
            let mut user_input = std::string::String::new();

            stdin()
                .read_line(&mut user_input)
                .expect("Didn't enter a correct string");
            Ok(DataType::String(user_input))
        }
    },
};

#[cfg(target_arch = "wasm32")]
pub const INPUT: CoreFunction = CoreFunction {
    id: "input",
    func: |values: &[DataType]| {
        if let Some(DataType::String(string)) = values.get(0) {
            let result = prompt(&string);
            match result {
                Some(res) => Ok(DataType::String(res)),
                None => Ok(DataType::String("".to_string())),
            }
        } else {
            let result = prompt("");

            match result {
                Some(res) => Ok(DataType::String(res)),
                None => Ok(DataType::String("".to_string())),
            }
        }
    },
};
