use std::{cell::RefCell, collections::HashMap, ptr::addr_of, rc::Rc};

use crate::evaluator::{Environment, EvalError, eval};

#[derive(Clone)]
pub struct Closure {
    pub ast: Box<DataType>,
    pub params: Vec<String>,
    pub env: Rc<RefCell<Environment>>,
}

impl Closure {
    pub fn func(&self, args: &[DataType]) -> Result<DataType, EvalError> {
        let mut i = 0;

        loop {
            let (name, param) = match (self.params.get(i), args.get(i)) {
                (Some(ampersand), Some(_)) if ampersand == "&" => {
                    let Some(name) = self.params.get(i + 1) else {
                        return Err(EvalError {
                            msg: "& found in closure without variadic argument name".to_string(),
                        });
                    };

                    let mut children = vec![];
                    let mut i2 = 0;

                    loop {
                        let Some(param) = args.get(i2) else {
                            break;
                        };
                        children.push(param.clone());
                        i2 += 1;
                    }

                    self.env
                        .borrow_mut()
                        .set(name.to_owned(), DataType::List(children));

                    break;
                }

                (Some(name), Some(param)) => (name, param),

                (None, None) => {
                    break;
                }

                _ => {
                    return Err(EvalError {
                        msg: "Parameters given do not match expected parameters".to_string(),
                    });
                }
            };

            self.env.borrow_mut().set(name.to_owned(), param.clone());
            i += 1;
        }

        eval(&self.ast, self.env.clone())
    }
}

#[derive(Clone)]
pub enum DataType {
    Nil(),
    List(Vec<DataType>),
    Symbol(std::string::String),
    Integer(i128),
    Bool(bool),
    Float(f64),
    String(String),
    Comment(),
    Vector(Vec<DataType>),
    Dictionary(HashMap<String, DataType>),
    Closure(Closure),
    NativeFunction((i8, &'static fn(&[DataType]) -> Result<DataType, EvalError>)),
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Symbol(l0), Self::Symbol(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Vector(l0), Self::Vector(r0)) => l0 == r0,
            (Self::Dictionary(l0), Self::Dictionary(r0)) => l0 == r0,
            (Self::Closure(l0), Self::Closure(r0)) => addr_of!(l0) == addr_of!(r0),
            (Self::NativeFunction(l0), Self::NativeFunction(r0)) => r0.0 == l0.0,
            _ => false,
        }
    }
}

impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::List(vector) => write!(
                f,
                "({})",
                vector
                    .iter()
                    .map(|data_type| format!("{:?}", data_type))
                    .collect::<Vec<std::string::String>>()
                    .join(" ")
            ),
            DataType::Vector(vector) => write!(
                f,
                "[{}]",
                vector
                    .iter()
                    .map(|data_type| format!("{:?}", data_type))
                    .collect::<Vec<std::string::String>>()
                    .join(" ")
            ),
            DataType::Dictionary(dict) => write!(
                f,
                "{{{}}}",
                dict.iter()
                    .map(|value| format!("{}: {:?}", value.0, value.1))
                    .collect::<Vec<std::string::String>>()
                    .join(", ")
            ),
            DataType::Symbol(symbol) => write!(f, "{}", symbol),
            DataType::Comment() => write!(f, ""),
            DataType::Nil() => write!(f, "nil"),
            DataType::Bool(value) => write!(f, "{}", value),
            DataType::Float(float) => write!(f, "{}", float),
            DataType::Integer(num) => write!(f, "{}", num),
            DataType::String(str) => write!(f, "\"{}\"", str),
            DataType::Closure(func) => write!(f, "Closure: {:p}", func),
            DataType::NativeFunction(func) => write!(f, "Native Function: {:p}", func),
        }
    }
}
