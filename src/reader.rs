use std::collections::{HashMap, VecDeque};

use DataType::*;
use DataTypeHashable::*;
use regex::Regex;

/*
[\s,]* get rid of whitespaces and commas
~@ match special character combination
[\[\]{}()'`~^@] match any special characters
"(?:\\.|[^\\"])*"? match strings with escaped characters (can be unbalanced)
;.* match comments starting with ;
[^\s\[\]{}('"`,;)]* match anything else
*/
pub const REGEX_TOKEN_EXP: &str =
    r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;

pub struct Reader {
    tokens: VecDeque<std::string::String>,
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: std::string::String,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DataTypeHashable {
    Number(i128),
    String(std::string::String),
}

#[derive(Clone)]
pub enum DataType {
    Nil(),
    Hashable(DataTypeHashable),
    Node(Vec<DataType>),
    Comment(),
    List(Vec<DataType>),
    Dictionary(HashMap<DataTypeHashable, DataType>),
    Symbol(std::string::String),
    Bool(bool),
    Float(f64),
}

impl std::fmt::Debug for DataTypeHashable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(number) => write!(f, "{}", number),
            String(string) => write!(f, "\"{}\"", string),
        }
    }
}

impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hashable(primitive) => write!(f, "{:?}", primitive),
            Node(vector) => write!(
                f,
                "({})",
                vector
                    .iter()
                    .map(|data_type| format!("{:?}", data_type))
                    .collect::<Vec<std::string::String>>()
                    .join(" ")
            ),
            List(vector) => write!(
                f,
                "[{}]",
                vector
                    .iter()
                    .map(|data_type| format!("{:?}", data_type))
                    .collect::<Vec<std::string::String>>()
                    .join(" ")
            ),
            Dictionary(dict) => write!(
                f,
                "{{{}}}",
                dict.iter()
                    .map(|value| format!("{:?}: {:?}", value.0, value.1))
                    .collect::<Vec<std::string::String>>()
                    .join(", ")
            ),
            Symbol(symbol) => write!(f, "{}", symbol),
            Comment() => write!(f, ""),
            Nil() => write!(f, "nil"),
            Bool(value) => write!(f, "{}", value),
            Float(float) => write!(f, "{}", float),
        }
    }
}

impl Reader {
    pub fn next(&mut self) -> Option<std::string::String> {
        self.tokens.pop_front()
    }

    pub fn peek(&self) -> Option<std::string::String> {
        self.tokens.get(0).cloned()
    }

    pub fn new(input: VecDeque<std::string::String>) -> Reader {
        Reader { tokens: input }
    }

    pub fn read(&mut self) -> Result<DataType, ParseError> {
        let Some(current) = self.peek() else {
            return Err(ParseError {
                msg: "Reading empty string".to_string(),
            });
        };

        if current == "(" {
            self.next();
            self.read_list(")".to_string())
        } else {
            self.read_atom()
        }
    }

    pub fn read_list(
        &mut self,
        end_character: std::string::String,
    ) -> Result<DataType, ParseError> {
        let mut children = vec![];

        loop {
            let token = match self.peek() {
                Some(t) => t,
                None => {
                    return Err(ParseError {
                        msg: "Couldn't find closing bracket".to_string(),
                    });
                }
            };

            if token == end_character {
                break;
            }

            children.push(self.read()?);
        }
        self.next();
        if end_character == ")" {
            return Ok(List(children));
        } else {
            return Ok(Node(children));
        }
    }

    pub fn read_dictionary(
        &mut self,
        end_character: std::string::String,
    ) -> Result<DataType, ParseError> {
        let mut children: HashMap<DataTypeHashable, DataType> = HashMap::new();

        loop {
            let token = match self.peek() {
                Some(t) => t,
                None => {
                    return Err(ParseError {
                        msg: "Couldn't find closing bracket".to_string(),
                    });
                }
            };

            if token == end_character {
                break;
            }

            let child1 = match self.read() {
                Ok(Hashable(p)) => p,
                Ok(_) => {
                    return Err(ParseError {
                        msg: "Cannot use non-hashable as dictionary key!".to_string(),
                    });
                }
                Err(_) => {
                    return Err(ParseError {
                        msg: "Couldn't find closing bracket".to_string(),
                    });
                }
            };

            let child2 = self.read()?;

            children.insert(child1, child2);
        }
        self.next();
        return Ok(Dictionary(children));
    }

    pub fn read_atom(&mut self) -> Result<DataType, ParseError> {
        let Some(token) = self.next() else {
            return Err(ParseError {
                msg: "Unexpected lack of atom!".to_string(),
            });
        };

        if let Ok(number) = token.parse::<i128>() {
            Ok(Hashable(Number(number)))
        } else if let Ok(number) = token.parse::<f64>() {
            Ok(Float(number))
        } else {
            let Some(first_char) = token.chars().nth(0) else {
                return Err(ParseError {
                    msg: "Unexpected empty atom!".to_string(),
                });
            };
            match token.as_str() {
                "false" => Ok(Bool(false)),
                "true" => Ok(Bool(true)),
                "nil" => Ok(Nil()),
                _ if first_char == '"' => {
                    let converted = token
                        .replace("\\n", "\n")
                        .replace("\\\\", "\\")
                        .replace("\\\"", "\"");
                    let quotes_removed = &converted[1..converted.len() - 1];
                    Ok(Hashable(String(quotes_removed.to_string())))
                }
                _ if first_char == ';' => Ok(Comment()),
                _ if first_char == '[' => self.read_list("]".to_string()),
                _ if first_char == '{' => self.read_dictionary("}".to_string()),
                _ => Ok(Symbol(token)),
            }
        }
    }
}

pub fn get_regex() -> Regex {
    Regex::new(REGEX_TOKEN_EXP).expect("Expected a valid regex expression!")
}

pub fn tokenize(input: std::string::String, re: Regex) -> VecDeque<std::string::String> {
    let mut tokens: VecDeque<std::string::String> = VecDeque::new();
    for (_, [token_match]) in re
        .captures_iter(input.trim())
        .map(|captures| captures.extract())
    {
        tokens.push_back(token_match.to_string());
    }
    tokens
}
