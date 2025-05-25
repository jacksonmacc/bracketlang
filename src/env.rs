use crate::{
    EvalError,
    reader::{DataType, DataType::*, DataTypeHashable::*},
};

pub struct Symbol {
    pub id: &'static str,
    pub func: fn(&[DataType]) -> Result<DataType, EvalError>,
}

pub const ADDITION: Symbol = Symbol {
    id: "+",
    func: |values: &[DataType]| {
        if values.len() > 1 {
            match values.first() {
                Some(Hashable(Number(_))) => {
                    let mut total = 0;
                    for value in values {
                        match value {
                            Hashable(Number(num)) => total += num,
                            _ => {
                                return Err(EvalError {
                                    msg: "Invalid types for addition!".to_string(),
                                });
                            }
                        }
                    }

                    Ok(Hashable(Number(total)))
                }
                Some(Hashable(String(_))) => {
                    let mut total = std::string::String::new();
                    for value in values {
                        match value {
                            Hashable(String(string)) => total += string,
                            _ => {
                                return Err(EvalError {
                                    msg: "Invalid types for addition!".to_string(),
                                });
                            }
                        }
                    }

                    Ok(Hashable(String(total)))
                }
                Some(Float(_)) => {
                    let mut total = 0.0;
                    for value in values {
                        match value {
                            Float(float) => total += float,
                            _ => {
                                return Err(EvalError {
                                    msg: "Invalid types for addition!".to_string(),
                                });
                            }
                        }
                    }

                    Ok(Float(total))
                }
                None | Some(_) => Err(EvalError {
                    msg: "Invalid types for addition".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Not enough arguments for addition".to_string(),
            })
        }
    },
};

pub const MULTIPLICATION: Symbol = Symbol {
    id: "*",
    func: |values: &[DataType]| {
        if values.len() > 1 {
            match values.first() {
                Some(Hashable(Number(_))) => {
                    let mut total = 1;
                    for value in values {
                        match value {
                            Hashable(Number(num)) => total *= num,
                            _ => {
                                return Err(EvalError {
                                    msg: "Invalid types for mutliplication!".to_string(),
                                });
                            }
                        }
                    }

                    Ok(Hashable(Number(total)))
                }
                Some(Float(_)) => {
                    let mut total = 1.0;
                    for value in values {
                        match value {
                            Float(float) => total *= float,
                            _ => {
                                return Err(EvalError {
                                    msg: "Invalid types for multiplication!".to_string(),
                                });
                            }
                        }
                    }

                    Ok(Float(total))
                }
                None | Some(_) => Err(EvalError {
                    msg: "Invalid types for multiplication".to_string(),
                }),
            }
        } else {
            Err(EvalError {
                msg: "Not enough arguments for multiplication".to_string(),
            })
        }
    },
};

pub const SUBTRACTION: Symbol = Symbol {
    id: "-",
    func: |values: &[DataType]| {
        if values.len() == 2 {
            match (values.get(0), values.get(1)) {
                (Some(Hashable(Number(num1))), Some(Hashable(Number(num2)))) => {
                    Ok(Hashable(Number(num1 - num2)))
                }

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
                (Some(Hashable(Number(num1))), Some(Hashable(Number(num2)))) => {
                    Ok(Hashable(Number(num1 / num2)))
                }

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
