use std::{cell::RefCell, rc::Rc};

use evaluator::eval;
use reader::{ParseError, Reader, get_regex, tokenize};
use variable_type::DataType;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{env::*, variable_type::Environment};

mod env;
mod evaluator;
mod reader;
pub mod variable_type;

#[cfg(test)]
mod tests;

fn read(input: String) -> Result<DataType, ParseError> {
    let tokens = tokenize(input, get_regex());
    let mut reader = Reader::new(tokens);
    reader.read()
}

fn print(input: DataType) -> String {
    format!("{:?}", input)
}

pub fn run_preamble(preamble: &str, repl_env: Rc<RefCell<Environment>>) {
    match re(preamble.to_string(), repl_env.clone()) {
        Err(e) => {
            println!("Error in core function definition! {}", e);
            return;
        }
        _ => (),
    };
}

#[wasm_bindgen]
pub struct EnvironmentHolder {
    env: Rc<RefCell<Environment>>,
}

impl EnvironmentHolder {
    pub fn get(&self) -> Rc<RefCell<Environment>> {
        return self.env.clone();
    }
}

#[wasm_bindgen]
pub fn create_default_env() -> EnvironmentHolder {
    let repl_env = create_default_repl_env();

    run_preamble("(def! not (fn* (a) (if a false true)))", repl_env.clone());
    run_preamble(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\\nnil)\")))))",
        repl_env.clone(),
    );
    run_preamble(
        "(defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw \"odd number of forms to cond\")) (cons 'cond (rest (rest xs)))))))",
        repl_env.clone(),
    );

    EnvironmentHolder { env: repl_env }
}

#[wasm_bindgen]
pub fn evaluate_string(input: &str, env: &mut EnvironmentHolder) -> Option<String> {
    rep(input.to_string(), env.get())
}

pub fn rep(input: String, repl_env: Rc<RefCell<Environment>>) -> Option<String> {
    let ast = match read(input) {
        Ok(r) => r,
        Err(e) => return Some(format!("PARSE ERROR: {}", e.msg)),
    };

    let eval_result = match eval(&ast, repl_env.clone(), repl_env.clone()) {
        Ok(r) => r,
        Err(e) => return Some(format!("RUNTIME ERROR: {}", e.msg)),
    };

    if let DataType::Nil() = eval_result {
        None
    } else {
        let print_result = print(eval_result);
        Some(print_result)
    }
}

pub fn re(input: String, repl_env: Rc<RefCell<Environment>>) -> Result<DataType, String> {
    let ast = match read(input) {
        Ok(r) => r,
        Err(e) => return Err(e.msg),
    };

    let result = match eval(&ast, repl_env.clone(), repl_env.clone()) {
        Ok(r) => r,
        Err(e) => return Err(e.msg),
    };

    Ok(result)
}

#[allow(unused_assignments)]
pub fn create_default_repl_env() -> Rc<RefCell<Environment>> {
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
        CHECK_SYMBOL,
        CHECK_VECTOR,
        CHECK_DICTIONARY,
        CHECK_SEQUENTIAL,
        SYMBOL,
        DICTIONARY,
        VECTOR,
        ASSOC,
        DISSOC,
        GET,
        CONTAINS,
        KEYS,
        VALUES,
        CHECK_STR,
        CHECK_INTEGER,
        CHECK_FLOAT,
        CHECK_FN,
        CHECK_MACRO,
        TIME_MS
    );

    Rc::new(RefCell::new(repl_env))
}
