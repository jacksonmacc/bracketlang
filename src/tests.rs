use std::collections::VecDeque;

use crate::{
    create_repl_env, eval, read,
    reader::{DataType, DataTypeHashable, get_regex, tokenize},
};

const FIB_TEST: &str = "(defun fib (n)
  \"Return the nth Fibonacci number.\"
  (if (< n 2)
      n
      (+ (fib (- n 1))
         (fib (- n 2)))))
";

#[test]
fn test_tokenize_simple_addition() {
    let tokens = tokenize("(+ 3 2)".to_string(), get_regex());
    let mut expected_tokens = VecDeque::new();

    for token in vec!["(", "+", "3", "2", ")"] {
        expected_tokens.push_back(token.to_string());
    }

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenize_fibonacci() {
    let tokens = tokenize(FIB_TEST.to_string(), get_regex());
    let mut expected_tokens = VecDeque::new();

    for token in vec![
        "(",
        "defun",
        "fib",
        "(",
        "n",
        ")",
        "\"Return the nth Fibonacci number.\"",
        "(",
        "if",
        "(",
        "<",
        "n",
        "2",
        ")",
        "n",
        "(",
        "+",
        "(",
        "fib",
        "(",
        "-",
        "n",
        "1",
        ")",
        ")",
        "(",
        "fib",
        "(",
        "-",
        "n",
        "2",
        ")",
        ")",
        ")",
        ")",
        ")",
    ] {
        expected_tokens.push_back(token.to_string());
    }

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_parsing_basic() {
    let ast = read("(+ 3 2)".to_string()).unwrap();
    assert_eq!(format!("{:?}", ast), "(+ 3 2)");
}

#[test]
fn test_parsing_fibonacci() {
    let ast = read(FIB_TEST.to_string()).unwrap();
    assert_eq!(
        format!("{:?}", ast),
        "(defun fib (n) \"Return the nth Fibonacci number.\" (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))"
    );
}

#[test]
fn test_parsing_list() {
    let ast = read("[1 2 3]".to_string()).unwrap();
    assert_eq!(format!("{:?}", ast), "[1 2 3]");
}

#[test]
fn test_parsing_dict() {
    let ast = read("{\"hello\" 1 \"world\" 2}".to_string()).unwrap();
    assert_eq!(format!("{:?}", ast), "{\"hello\": 1, \"world\": 2}");
}

#[test]
fn test_eval_simple_addition() {
    let result = eval(&read("(+ 3 2)".to_string()).unwrap(), &create_repl_env()).unwrap();
    if let DataType::Hashable(num_result) = result {
        assert_eq!(num_result, DataTypeHashable::Number(5));
    } else {
        assert!(false);
    }
}
