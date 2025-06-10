use std::collections::{HashMap, VecDeque};

use regex::Regex;

use crate::{env::DEREF, variable_type::DataType};

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
            return Ok(DataType::List(children));
        } else {
            return Ok(DataType::Vector(children));
        }
    }

    pub fn read_dictionary(
        &mut self,
        end_character: std::string::String,
    ) -> Result<DataType, ParseError> {
        let mut children: HashMap<String, DataType> = HashMap::new();

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
                Ok(p) => p,
                Err(_) => {
                    return Err(ParseError {
                        msg: "Couldn't find closing bracket".to_string(),
                    });
                }
            };

            let child2 = self.read()?;

            children.insert(format!("{:?}", child1), child2);
        }
        self.next();
        return Ok(DataType::Dictionary(children));
    }

    pub fn read_atom(&mut self) -> Result<DataType, ParseError> {
        let Some(token) = self.next() else {
            return Err(ParseError {
                msg: "Unexpected lack of atom!".to_string(),
            });
        };

        if let Ok(number) = token.parse::<i128>() {
            Ok(DataType::Integer(number))
        } else if let Ok(number) = token.parse::<f64>() {
            Ok(DataType::Float(number))
        } else {
            let Some(first_char) = token.chars().nth(0) else {
                return Err(ParseError {
                    msg: "Unexpected empty atom!".to_string(),
                });
            };
            match token.as_str() {
                "false" => Ok(DataType::Bool(false)),
                "true" => Ok(DataType::Bool(true)),
                "nil" => Ok(DataType::Nil()),
                "@" => Ok(DataType::List(vec![
                    DataType::Symbol(DEREF.id.to_string()),
                    self.read_atom()?,
                ])),
                "'" => Ok(DataType::List(vec![
                    DataType::Symbol("quote".to_string()),
                    self.read()?,
                ])),
                "`" => Ok(DataType::List(vec![
                    DataType::Symbol("quasiquote".to_string()),
                    self.read()?,
                ])),
                "~" => Ok(DataType::List(vec![
                    DataType::Symbol("unquote".to_string()),
                    self.read()?,
                ])),
                "~@" => Ok(DataType::List(vec![
                    DataType::Symbol("splice-unquote".to_string()),
                    self.read()?,
                ])),
                _ if first_char == '"' => {
                    let converted = token
                        .replace("\\n", "\n")
                        .replace("\\\\", "\\")
                        .replace("\\\"", "\"");
                    let quotes_removed = &converted[1..converted.len() - 1];
                    Ok(DataType::String(quotes_removed.to_string()))
                }
                _ if first_char == ';' => Ok(DataType::Comment()),
                _ if first_char == '[' => self.read_list("]".to_string()),
                _ if first_char == '{' => self.read_dictionary("}".to_string()),
                _ => Ok(DataType::Symbol(token)),
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
