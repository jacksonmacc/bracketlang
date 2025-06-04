use std::collections::VecDeque;

use crate::*;

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
    let test = format!("{:?}", ast);
    println!("{}", test);
    assert!(
        format!("{:?}", ast).contains("\"hello\": 1")
            && format!("{:?}", ast).contains("\"world\": 2")
    );
}

#[test]
fn test_eval_simple_addition() {
    let result = eval(&read("(+ 3 2)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_addition_with_strings() {
    let result = eval(
        &read("(+ \"Hello, \" \"World!\")".to_string()).unwrap(),
        create_repl_env(),
    )
    .unwrap();
    if let DataType::String(num_result) = result {
        assert_eq!(num_result, "Hello, World!".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_addition_with_negatives() {
    let result = eval(&read("(+ 3 -4)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, -1);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_addition_with_floats() {
    let result = eval(
        &read("(+ 3.0 -4.5)".to_string()).unwrap(),
        create_repl_env(),
    )
    .unwrap();
    if let DataType::Float(float) = result {
        assert_eq!(float, -1.5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_subtraction() {
    let result = eval(&read("(- 3 2)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 1);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_subtraction_with_negatives() {
    let result = eval(&read("(- 3 -4)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 7);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_subtraction_with_floats() {
    let result = eval(&read("(- 2.5 3.5)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Float(float) = result {
        assert_eq!(float, -1.0);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_multiplication() {
    let result = eval(&read("(* 3 2)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 6);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_multiplication_with_negatives() {
    let result = eval(&read("(* 3 -4)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, -12);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_multiplication_with_floats() {
    let result = eval(&read("(* 3.0 1.5)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Float(float) = result {
        assert_eq!(float, 4.5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_division() {
    let result = eval(&read("(/ 4 2)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 2);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_division_with_negatives() {
    let result = eval(&read("(/ -18 -6)".to_string()).unwrap(), create_repl_env()).unwrap();
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_division_with_floats() {
    let result = eval(
        &read("(/ 2.25 1.5)".to_string()).unwrap(),
        create_repl_env(),
    )
    .unwrap();
    if let DataType::Float(float) = result {
        assert_eq!(float, 1.5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_def() {
    let env = create_repl_env();
    let _ = eval(&read("(def! a 3)".to_string()).unwrap(), env.clone()).unwrap();
    let result = eval(&read("a".to_string()).unwrap(), env.clone()).unwrap();
    if let DataType::Integer(int) = result {
        assert_eq!(int, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_let() {
    let env = create_repl_env();
    let result = eval(&read("(let* (c 3) c)".to_string()).unwrap(), env.clone()).unwrap();
    if let DataType::Integer(int) = result {
        assert_eq!(int, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_do() {
    let env = create_repl_env();
    let result = eval(&read("(do 1 2 3 4)".to_string()).unwrap(), env.clone()).unwrap();
    if let DataType::Integer(int) = result {
        assert_eq!(int, 4);
    } else {
        assert!(false);
    }
}
