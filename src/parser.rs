
use lexer::Token;
use lexer::Keyword;
use lexer::Operator;
use evaluator::Expression;
use environment::Environment;
use predicate::Predicate;

fn parse_conditional(mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {

    // after "if keyword, expect parenthesis
    match tokens.remove(0) { Token::LParen => {}, _ => panic!("expected left parenthesis")};
   
    // after lparen, expect operator
    let operator = match tokens.remove(0) {
        Token::Oper(o) => o,
        _                  => panic!("expected operator in conditional"),
    };

    // left value & right value
    let left_val = parse(tokens, environment);
    let right_val = parse(tokens, environment);

    // after values, right paren
    match tokens.remove(0) { Token::RParen => {}, _ => panic!("expected right parenthesis") };

    // if true and if false
    let if_true = parse(tokens, environment);
    let if_false = parse(tokens, environment);

    // final right parenthesis
    match tokens.remove(0) { Token::RParen => {}, _ => panic!("expected right parenthesis") };

    let predicate = Predicate {
        operator: operator,
        l_hand: left_val,
        r_hand: right_val,
        if_true: if_true,
        if_false: if_false,
    };

    return Expression::Predicate(Box::new(predicate));
}

fn parse_function(fname: String, mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {
	let func = environment.functions.get(&fname);
	let mut args = Vec::<Expression>::new();
	while tokens.len() > 1 { // yeah, this doesn't work...
		match tokens.get(0).unwrap() { 
			&Token::RParen => (break),
			_ => {args.push(parse(tokens,environment))},
		}
	}
    match tokens.remove(0) { Token::RParen => {}, _ => panic!("expected right parenthesis") };
	match func {
		Some(f)  => f.parse(args, environment),
		_        => panic!("unknown function name {}", fname),
	}
}

fn parse_compound(mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {
	
    let operator = tokens.remove(0);
	// does the first token exist in our environment?

    // the other possibilities are for the first token to be an operator OR keyword
    let c = match operator {
        Token::Oper(o) => o,
        Token::Keyword(keyword) => match keyword { Keyword::If => {return parse_conditional(tokens, environment)}, _ => panic!("unexpected keyword") },

		Token::Constant(s) => { return parse_function(s,tokens,environment); },
        _              => panic!("Unknown token {:?}",operator),
    };

    // get vector of following expressions
    let mut expressions = Vec::<Expression>::new();
    loop {
        match tokens.get(0).unwrap() {
            &Token::RParen     => break,
            _                  => expressions.push(parse(tokens, environment)),
        }
    }
    // after breaking, remove last element
    tokens.remove(0);

    // create a token, must be + or *
    match c {
        Operator::Plus  => Expression::Plus(expressions),
        Operator::Mult  => Expression::Mult(expressions),
        _               => panic!("unexpected operator"),
    }

}

fn constant_to_expression(s: String, environment: &Environment) -> Expression {
	let expr = environment.variables.get(&s);
	match expr {
		Some(a) => a.clone(),
		_ => Expression::Number(s.parse::<f64>().unwrap()),
	}
}

pub fn parse(mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {
    // if integer, return integer
    let tok = tokens.remove(0);
    match tok {
        Token::Constant(s) => constant_to_expression(s, environment),
        Token::LParen      => parse_compound(tokens, environment),
        _ => panic!("Unexpected token in parse, {:?}", tok),
    }

}

