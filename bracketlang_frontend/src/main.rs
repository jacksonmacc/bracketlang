use bracketlang_backend::{self, create_default_repl_env, re, rep, variable_type::{DataType, Environment}};
use std::{
    cell::RefCell,
    io::{Write, stdin, stdout},
    rc::Rc,
};

fn run_preamble(preamble: &str, repl_env: Rc<RefCell<Environment>>) {
    match re(preamble.to_string(), repl_env.clone()) {
        Err(e) => {
            println!("Error in core function definition! {}", e);
            return;
        }
        _ => (),
    };
}

fn main() {
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

    let args: Vec<String> = std::env::args().collect();
    if let Some(filename) = args.get(1) {
        let mut repl_args = vec![];
        for arg in &args[2..] {
            repl_args.push(DataType::String(arg.clone()));
        }
        repl_env
            .borrow_mut()
            .set("*ARGV*".to_string(), DataType::List(repl_args));
        match rep(format!("(load-file \"{}\")", filename), repl_env.clone()) {
            Some(res) => {
                println!("Error: {}", res);
                return;
            }
            None => return,
        }
    }

    loop {
        print!("user> ");
        stdout()
            .flush()
            .expect("Flushing stdout should have worked.");
        let mut user_input = String::new();

        let user_input_result = stdin()
            .read_line(&mut user_input)
            .expect("Didn't enter a correct string");

        // Recieved EOF
        if user_input_result == 0 {
            println!();
            println!("Quitting...");
            break;
        }

        if let Some(result) = rep(user_input, repl_env.clone()) {
            println!("{}", result);
        }
    }
}
