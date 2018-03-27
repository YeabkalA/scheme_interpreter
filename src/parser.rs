
use lexer::Token;
use lexer::Keyword;
use lexer::Operator;
use lexer::CompoundPredicateType;
use evaluator::Expression;
use environment::Environment;
use predicate::Predicate;
use predicate::CompoundPredicate;

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

fn parse_simple_predicate(mut tokens: &mut Vec<Token>, environment: &Environment) -> Predicate {
	
	// swallow the opening parenthesis.
	match tokens.remove(0) { Token::LParen => {}, _ => panic!("expected left parenthesis")};
	
	// three elements. A conditional operator, an expression, and another expression.
	let operator = match tokens.remove(0) {
		Token::Oper(o) => match o {
			Operator::Greater => o,
			Operator::Less => o,
			Operator::Equal => o,
			_ => panic!("expected an operator, specifically <>="),
		},
		_ => panic!("expected an operator!"),
	};
	
	let left_expression = parse(tokens, environment);
	let right_expression = parse(tokens, environment);

	// swallow the closing parenthesis
	match tokens.remove(0) { Token::RParen => {}, _ => panic!("expected right parenthesis")};
	
	// two "fake" expressions for if-true and if-false
	let fake_if_true = Expression::Number(1.0);
	let fake_if_false = Expression::Number(0.0);
	
	// create the predicate
	Predicate {
		operator: operator,
		l_hand: left_expression,
		r_hand: right_expression,
		if_true: fake_if_true,
		if_false: fake_if_false,
	}
}

fn parse_compound_predicate(ct: CompoundPredicateType, mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {

	// swallow a left parenthesis, which will be the first predicate in the list.
	match tokens.remove(0) { Token::LParen => {}, _ => panic!("expected left parenthesis")}
	
	// hold onto the list of predicates
	let mut list_of_predicates = Vec::<Predicate>::new();
	
	// eat "simple predicates" until we hit a right parenthesis.
	loop {
		let simple = parse_simple_predicate(tokens, environment);
		list_of_predicates.push(simple);
		match tokens[0] {
			Token::RParen => break,
			_ => {},
		}
	}
	// eat the right paren after the list of simple predicates
	match tokens.remove(0) { Token::RParen => {}, _ => panic!("expected right parenthesis")}
	
	// if-true expression
	let if_true = parse(tokens, environment);
	
	// if-false expresssion
	let if_false = parse(tokens, environment);
	
	// final right parenthesis
	match tokens.remove(0) { Token::RParen => {}, _ => panic!("expected right parenthesis")}

	// create our returned expression
	let compound_predicate = CompoundPredicate {
		compound: ct,
		list: list_of_predicates,
		if_true: if_true,
		if_false: if_false,
	};
	Expression::CompoundPredicate(Box::new(compound_predicate))

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
		Token::CompoundPredicate(compound) => { return parse_compound_predicate(compound,tokens,environment); },
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

