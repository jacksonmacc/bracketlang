use std::collections::VecDeque;

use crate::*;

fn run_line(string: &str, env: Rc<RefCell<Environment>>) -> DataType {
    eval(&read(string.to_string()).unwrap(), env.clone(), env.clone()).unwrap()
}

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
    let env = create_default_repl_env();
    let result = run_line("(+ 3 2)", env);
    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_addition_with_strings() {
    let env = create_default_repl_env();
    let result = run_line("(+ \"Hello, \" \"World!\")", env);
    if let DataType::String(num_result) = result {
        assert_eq!(num_result, "Hello, World!".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_addition_with_negatives() {
    let env = create_default_repl_env();
    let result = run_line("(+ 3 -4)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, -1);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_addition_with_floats() {
    let env = create_default_repl_env();
    let result = run_line("(+ 3.0 -4.5)", env);

    if let DataType::Float(float) = result {
        assert_eq!(float, -1.5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_subtraction() {
    let env = create_default_repl_env();
    let result = run_line("(- 3 2)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 1);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_subtraction_with_negatives() {
    let env = create_default_repl_env();
    let result = run_line("(- 3 -4)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 7);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_subtraction_with_floats() {
    let env = create_default_repl_env();
    let result = run_line("(- 2.5 3.5)", env);

    if let DataType::Float(float) = result {
        assert_eq!(float, -1.0);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_multiplication() {
    let env = create_default_repl_env();
    let result = run_line("(* 3 2)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 6);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_multiplication_with_negatives() {
    let env = create_default_repl_env();
    let result = run_line("(* 3 -4)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, -12);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_multiplication_with_floats() {
    let env = create_default_repl_env();
    let result = run_line("(* 3.0 1.5)", env);

    if let DataType::Float(float) = result {
        assert_eq!(float, 4.5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_simple_division() {
    let env = create_default_repl_env();
    let result = run_line("(/ 4 2)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 2);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_division_with_negatives() {
    let env = create_default_repl_env();
    let result = run_line("(/ -18 -6)", env);

    if let DataType::Integer(num_result) = result {
        assert_eq!(num_result, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_eval_division_with_floats() {
    let env = create_default_repl_env();
    let result = run_line("(/ 2.25 1.5)", env);

    if let DataType::Float(float) = result {
        assert_eq!(float, 1.5);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_def() {
    let env = create_default_repl_env();
    let _ = run_line("(def! a 3)", env.clone());
    let result = run_line("a", env.clone());

    if let DataType::Integer(int) = result {
        assert_eq!(int, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_let() {
    let env = create_default_repl_env();
    let result = run_line("(let* (c 3) c)", env.clone());

    if let DataType::Integer(int) = result {
        assert_eq!(int, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_do() {
    let env = create_default_repl_env();
    let result = run_line("(do 1 2 3 4)", env.clone());

    if let DataType::Integer(int) = result {
        assert_eq!(int, 4);
    } else {
        assert!(false);
    }
}

#[test]
#[should_panic]
fn test_env_leak() {
    let env = create_default_repl_env();
    let result = run_line("(let* (c 18) c)", env.clone());

    if let DataType::Integer(int) = result {
        assert_eq!(int, 18);
    } else {
        assert!(false);
    }
    let _ = run_line("c", env.clone());
}

#[test]
fn test_simple_function() {
    let env = create_default_repl_env();
    let result = run_line("(def! x (fn* (a) a))", env.clone());

    let DataType::Closure(_) = result else {
        panic!()
    };
    let result = run_line("(x 3)", env.clone());

    if let DataType::Integer(int) = result {
        assert_eq!(int, 3);
    } else {
        assert!(false);
    }
}

#[test]
fn test_simple_load_file() {
    let env = create_default_repl_env();
    let _ = run_line(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\\nnil)\")))))",
        env.clone(),
    );
    let _ = run_line("(load-file \"test.bl\")", env.clone());
    let result = run_line("(inc4 3)", env.clone());

    if let DataType::Integer(int) = result {
        assert_eq!(int, 7);
    } else {
        assert!(false);
    }
}

#[test]
fn test_create_atom() {
    let env = create_default_repl_env();
    let result = run_line("(def! x (atom 3))", env.clone());

    let DataType::Atom(_) = result else {
        panic!();
    };

    let result = run_line("(deref x)", env.clone());
    if let DataType::Integer(int) = result {
        assert_eq!(int, 3);
    } else {
        panic!();
    }
}

#[test]
fn test_reset_atom() {
    let env = create_default_repl_env();
    let _ = run_line("(def! x (atom 3))", env.clone());
    let _ = run_line("(reset! x 4)", env.clone());

    let result = run_line("(deref x)", env.clone());
    if let DataType::Integer(int) = result {
        assert_eq!(int, 4);
    } else {
        panic!();
    }
}

#[test]
fn test_cons() {
    let env = create_default_repl_env();
    let result = run_line("(cons 1 (list 2 3))", env.clone());
    if let DataType::List(list) = result {
        assert_eq!(
            list,
            vec![
                DataType::Integer(1),
                DataType::Integer(2),
                DataType::Integer(3)
            ]
        );
    } else {
        panic!();
    }
}

#[test]
fn test_quote() {
    let env = create_default_repl_env();
    let result = run_line("(quote (b c))", env.clone());
    if let DataType::List(list) = result {
        assert_eq!(
            list,
            vec![
                DataType::Symbol("b".to_string()),
                DataType::Symbol("c".to_string())
            ]
        );
    } else {
        panic!();
    }
}

#[test]
fn test_quasiquote() {
    let env = create_default_repl_env();
    let result = run_line("(quasiquote (a lst d))", env.clone());

    if let DataType::List(list) = result {
        assert_eq!(
            list,
            vec![
                DataType::Symbol("a".to_string()),
                DataType::Symbol("lst".to_string()),
                DataType::Symbol("d".to_string())
            ]
        );
    } else {
        panic!();
    }
}

#[test]
fn test_quasiquote_unquote() {
    let env = create_default_repl_env();
    let _ = run_line("(def! lst (quote (b c)))", env.clone());
    let result = run_line("(quasiquote (a (unquote lst) d))", env.clone());

    if let DataType::List(list) = result {
        assert_eq!(
            list,
            vec![
                DataType::Symbol("a".to_string()),
                DataType::List(vec![
                    DataType::Symbol("b".to_string()),
                    DataType::Symbol("c".to_string())
                ]),
                DataType::Symbol("d".to_string())
            ]
        );
    } else {
        panic!();
    }
}

#[test]
fn test_splice_unquote() {
    let env = create_default_repl_env();
    let _ = run_line("(def! lst (quote (b c)))", env.clone());
    let result = run_line("(quasiquote (a (splice-unquote lst) d))", env.clone());

    if let DataType::List(list) = result {
        assert_eq!(
            list,
            vec![
                DataType::Symbol("a".to_string()),
                DataType::Symbol("b".to_string()),
                DataType::Symbol("c".to_string()),
                DataType::Symbol("d".to_string())
            ]
        );
    } else {
        panic!();
    }
}

#[test]
fn test_macros() {
    let env = create_default_repl_env();
    let _ = run_line(
        "(defmacro! unless (fn* (pred a b) `(if ~pred ~b ~a)))",
        env.clone(),
    );
    let result = run_line("(unless false 7 8)", env.clone());

    if let DataType::Integer(i) = result {
        assert_eq!(i, 7);
    } else {
        panic!();
    }
}

#[test]
fn test_macros_2() {
    let env = create_default_repl_env();
    let _ = run_line("(defmacro! makelist (fn* (x) x))", env.clone());
    let result = run_line("(makelist (+ 2 3))", env.clone());

    if let DataType::Integer(int) = result {
	assert_eq!(int, 5);
    } else {
        panic!();
    }
}
