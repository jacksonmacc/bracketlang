use std::{cell::RefCell, collections::HashMap, ptr::addr_of, rc::Rc};

use crate::evaluator::{Environment, RuntimeError};

#[derive(Clone)]
pub struct Closure {
    pub ast: Box<DataType>,
    pub params: Vec<String>,
    pub env: Rc<RefCell<Environment>>,
    pub repl_env: Rc<RefCell<Environment>>,
    pub is_macro: bool,
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
    NativeFunction((i8, &'static fn(&[DataType]) -> Result<DataType, RuntimeError>)),
    Atom(Rc<RefCell<DataType>>),
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
            (Self::Atom(l0), Self::Atom(r0)) => l0 == r0,
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
            DataType::Closure(func) => write!(f, "Closure({:p})", func),
            DataType::NativeFunction(func) => write!(f, "Fn{}", func.0),
            DataType::Atom(atom) => write!(f, "Atom({:p})", *atom),
        }
    }
}
