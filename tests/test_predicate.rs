
#[cfg(test)]

extern crate updated_scheme;

use updated_scheme::predicate::*;
use updated_scheme::evaluator::*;
use updated_scheme::interpreter::*;
use updated_scheme::lexer::*;

#[test]
fn test_raw_predicate() {

    let expr_1 = Expression::Number(1.0);
    let expr_2 = Expression::Number(2.0);
    let expr_3 = Expression::Number(3.0);
    let expr_4 = Expression::Number(4.0);

    let predicate = Predicate {
        operator: Operator::Greater, 
        l_hand: expr_1,
        r_hand: expr_2,
        if_true: expr_3,
        if_false: expr_4,
    };
    
    let result = predicate.evaluate();
    let value = match result {
        Expression::Number(a) => a,
        _ => panic!(),
    };

    assert_eq!(value, 4.0);

}


#[test]
fn test_with_multiple_variable_string() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (> x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 11.0);

}


#[test]
fn test_with_multiple_variable_string_2() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (< x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 30.0);

}


#[test]
fn test_with_multiple_variable_string_3() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (> x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 11.0);

}

#[test]
fn test_compound_predicate_raw_true_or() {
    let expr_1 = Expression::Number(1.0);
    let expr_2 = Expression::Number(2.0);
    let expr_3 = Expression::Number(3.0);
    let expr_4 = Expression::Number(4.0);

    let predicate = Predicate {
        operator: Operator::Greater, 
        l_hand: expr_1,
        r_hand: expr_2,
        if_true: expr_3,
        if_false: expr_4,
    };
	let compound = CompoundPredicate {
		compound: CompoundPredicateType::Or,
		list: vec![predicate],
		if_true: Expression::Number(1.0),
		if_false: Expression::Number(2.0),
	};
	let result = compound.evaluate();
    let value = match result {
        Expression::Number(a) => a,
        _ => panic!(),
    };
	assert_eq!(value, 2.0);
}

#[test]
fn test_compound_predicate_raw_false_or() {
    let expr_1 = Expression::Number(1.0);
    let expr_2 = Expression::Number(2.0);
    let expr_3 = Expression::Number(3.0);
    let expr_4 = Expression::Number(4.0);

    let predicate = Predicate {
        operator: Operator::Greater, 
        l_hand: expr_2,
        r_hand: expr_1,
        if_true: expr_3,
        if_false: expr_4,
    };
	let compound = CompoundPredicate {
		compound: CompoundPredicateType::Or,
		list: vec![predicate],
		if_true: Expression::Number(1.0),
		if_false: Expression::Number(2.0),
	};
	let result = compound.evaluate();
    let value = match result {
        Expression::Number(a) => a,
        _ => panic!(),
    };
	assert_eq!(value, 1.0);
}

#[test]
fn test_compound_or_from_string() {
    let env = String::from("(define (x 5)) (define (y 6)) (define (z 1))");
    let expr = String::from("(or ((< x y) (= x y)) z 0)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 1.0);
}

#[test]
fn test_compound_and_from_string() {
    let env = String::from("(define (x 5)) (define (y 6)) (define (z 1))");
    let expr = String::from("(and ((> x y) (= x y)) z 0)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 0.0);
}
#[test]
fn test_compound_not_from_string() {
    let env = String::from("(define (x 5)) (define (y 6)) (define (z 1))");
    let expr = String::from("(not ((> x y) (= x y)) z 0)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 1.0);
}