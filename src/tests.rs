use std::collections::VecDeque;

use crate::{
    read,
    reader::{get_regex, tokenize},
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
