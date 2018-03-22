#[cfg(test)]

extern crate updated_scheme;
use updated_scheme::predicate::*;
use updated_scheme::evaluator::*;
use updated_scheme::interpreter::*;
use updated_scheme::lexer::*;
use updated_scheme::function::*;
use updated_scheme::environment::*;
use updated_scheme::environment_parser::*;

#[test]
fn test_function_evaluation() {
	let mut subs = Vec::<String>::new();
	subs.push(String::from("x"));
	let mut vals = Vec::<Expression>::new();
	vals.push(Expression::Number(5.0));
	let f = Function {
		substitutions: subs,
		tokens: vec![Token::LParen,Token::Oper(Operator::Mult),
						Token::Constant(String::from("x")),Token::Constant(String::from("x")),
						Token::RParen]/* array of tokens */,
	};
	let mut envTokens = Vec::<Token>::new();
	let env = parse_to_environment(&mut envTokens);
	let v = f.parse(vals,&env);
	let val = evaluate(&v).ok().unwrap();
	assert_eq!(val,25.0);
}

#[test]
fn test_parse_to_function() {
	let string = String::from("(deffun (square x) (* x x))");
	let mut tok = tokenize(&string);
	let (n,f) = parse_to_function(&mut tok);
	assert_eq!(n,"square");
	assert_eq!(f.tokens.len(), 5);
	assert_eq!(f.substitutions.len(), 1);
}

#[test]
fn test_function_and_variable_in_environment() {
    let e = String::from("(deffun (square x) (* x x))(define (y 6))");
	let mut tok = tokenize(&e);
	let env = parse_to_environment(&mut tok);
	assert_eq!(env.variables.len(),1,"unexpected number of variables");
	assert_eq!(env.functions.len(),1,"unexpected number of functions");
	
}

#[test]
fn test_function_square() {

    let env = String::from("(deffun (square x) (* x x))(define (y 6))");
    let expr = String::from("(square y)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 36.0);
}

#[test]
fn test_sum() {
    let env = String::from("(deffun (square x) (* x x))(deffun (sum x y) (+ x y))");
    let expr = String::from("(sum (+ 3 3)(+ 4 4))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 14.0);
}

#[test]
fn test_square() {
    let env = String::from("(deffun (square x) (* x x))(deffun (sum x y) (+ x y))");
    let expr = String::from("(square (square (sum 1 1)))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 16.0);
}

#[test]
fn test_sum_of_squares() {
    let env = String::from("(deffun (square x) (* x x))(deffun (sum-of-squares a b) (+ (square a) (square b)))");
    let expr = String::from("(sum-of-squares 2 3)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 13.0);
}

#[test]
fn test_sum_of_squares_with_variables() {
    let env = String::from("(deffun (square x) (* x x))(deffun (sum-of-squares a b) (+ (square a) (square b)))(define (bees 2))(define (birds 3))");
    let expr = String::from("(sum-of-squares bees birds)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 13.0);
}

#[test]
fn test_function_variable_mangling() {
    let env = String::from("(deffun (square x) (* x x))(define (x 6))");
    let expr = String::from("(square x)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 36.0);
}
