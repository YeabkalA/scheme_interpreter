#[cfg(test)]

extern crate updated_scheme;
pub use updated_scheme::interpreter::*;
pub use updated_scheme::environment::*;
use std::collections::HashMap;

#[test]
fn test_single_number() {
    let val = interpret(String::from("88"));
    assert_eq!(val,88.0);
}

#[test]
fn test_negative_number() {
    let val = interpret(String::from("-10"));
    assert_eq!(val,-10.0);
}

#[test]
fn test_simple_evaluation() {
    let val = interpret(String::from("(+ 2 2)"));
    assert_eq!(val, 4.0);
}

#[test]
fn test_multiple_parens() {
    let val = interpret(String::from("(* (+ 7 6) (+ 2 3) (* 9 9))"));
    assert_eq!(val, 5265.0);
}

#[test]
fn test_with_environment_string() {

    let env = String::from("(define (x 5))");
    let expr = String::from("(+ x 5)");
    let val = interpret_with_environment_string(expr,env);
    assert_eq!(val, 10.0);

}

#[test]
fn test_with_multiple_variable_string() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(* x y)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 30.0);

}
