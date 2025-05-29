use crate::{
    EvalError,
    reader::{DataType, DataType::*},
};

pub struct Symbol {
    pub id: &'static str,
    pub func: fn(&[DataType]) -> Result<DataType, EvalError>,
}

pub const ADDITION: Symbol = Symbol {
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

pub const MULTIPLICATION: Symbol = Symbol {
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
pub const SUBTRACTION: Symbol = Symbol {
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

pub const DIVISION: Symbol = Symbol {
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
