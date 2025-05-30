use crate::{
    EvalError,
    reader::{DataType, DataType::*},
};

pub struct CoreFunction {
    pub id: &'static str,
    pub func: fn(&[DataType]) -> Result<DataType, EvalError>,
}

pub const ADDITION: CoreFunction = CoreFunction {
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

pub const MULTIPLICATION: CoreFunction = CoreFunction {
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
pub const SUBTRACTION: CoreFunction = CoreFunction {
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

pub const DIVISION: CoreFunction = CoreFunction {
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

pub const PRINT: CoreFunction = CoreFunction {
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

pub const LIST_CHECK: CoreFunction = CoreFunction {
    id: "list?",
    func: |values: &[DataType]| match values.first() {
        Some(DataType::List(_)) => Ok(DataType::Bool(true)),
        None => Err(EvalError {
            msg: "No arguments given to list?".to_string(),
        }),
        _ => Ok(DataType::Bool(false)),
    },
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
            Err(EvalError {
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

pub const EQUALS: CoreFunction = CoreFunction {
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
pub const GREATER_THAN: CoreFunction = CoreFunction {
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

pub const LESS_THAN: CoreFunction = CoreFunction {
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

pub const GREATER_THAN_OR_EQUALS: CoreFunction = CoreFunction {
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

pub const LESS_THAN_OR_EQUALS: CoreFunction = CoreFunction {
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
