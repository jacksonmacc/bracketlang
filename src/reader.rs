use std::collections::VecDeque;

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
    tokens: VecDeque<String>,
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
}

pub enum DataType {
    Node(Vec<DataType>),
    Number(u64),
    Symbol(String),
    String(String),
    Bool(bool),
    Nil(),
}

impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Symbol(symbol) => write!(f, "{}", symbol),
            DataType::Number(number) => write!(f, "{}", number),
            DataType::Node(vector) => write!(
                f,
                "({})",
                vector
                    .iter()
                    .map(|data_type| format!("{:?}", data_type))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            DataType::String(string) => write!(f, "\"{}\"", string),
            DataType::Bool(value) => write!(f, "{}", value),
            DataType::Nil() => write!(f, "nil"),
        }
    }
}

impl Reader {
    pub fn next(&mut self) -> Option<String> {
        self.tokens.pop_front()
    }

    pub fn peek(&self) -> Option<String> {
        self.tokens.get(0).cloned()
    }

    pub fn new(input: VecDeque<String>) -> Reader {
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
            self.read_list()
        } else {
            self.read_atom()
        }
    }

    pub fn read_list(&mut self) -> Result<DataType, ParseError> {
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

            if token == ")" {
                break;
            }

            let child = match self.read() {
                Ok(c) => c,
                Err(_) => {
                    return Err(ParseError {
                        msg: "Couldn't find closing bracket".to_string(),
                    });
                }
            };
            children.push(child);
        }
        self.next();
        return Ok(DataType::Node(children));
    }

    pub fn read_atom(&mut self) -> Result<DataType, ParseError> {
        let Some(token) = self.next() else {
            return Err(ParseError {
                msg: "Unexpected lack of atom!".to_string(),
            });
        };

        if let Ok(number) = token.parse::<u64>() {
            Ok(DataType::Number(number))
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
                _ if first_char == '"' => {
                    let converted = token.replace("\\n", "\n").replace("\\\\", "\\").replace("\\\"", "\"");
                    let quotes_removed = &converted[1..converted.len() - 1];
                    Ok(DataType::String(quotes_removed.to_string()))
                },
                _  => Ok(DataType::Symbol(token)),
            }
        }
    }
}

pub fn get_regex() -> Regex {
    Regex::new(REGEX_TOKEN_EXP).expect("Expected a valid regex expression!")
}

pub fn tokenize(input: String, re: Regex) -> VecDeque<String> {
    let mut tokens: VecDeque<String> = VecDeque::new();
    for (_, [token_match]) in re
        .captures_iter(input.trim())
        .map(|captures| captures.extract())
    {
        tokens.push_back(token_match.to_string());
    }
    tokens
}
