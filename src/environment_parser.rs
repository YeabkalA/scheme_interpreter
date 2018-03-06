
use lexer::*;
use parser::*;
use environment::*;
use evaluator::*;
use function::*;
use std::collections::HashMap;

pub fn parse_to_function(mut v: &mut Vec<Token>) -> (String,Function) {
	
	// functions begin with a lparen
	match v.remove(0) { Token::LParen => (), _ => panic!("A function definition must begin with a left paren") }
	
	// next is a "define"
	match v.remove(0) { Token::Keyword(k) => match k { Keyword::Deffun => {}, _ => panic!() }, _ => panic!(), }
	
	// another lparen
	match v.remove(0) { Token::LParen => (), _ => panic!("A function definition must begin with a left paren") }
	
	// function name
	let name = match v.remove(0) { Token::Constant(s) => s.clone(), _ => panic!("expecting a function name") };
	
	// get substitutions until a rparen.
	let mut subs = Vec::<String>::new();
	loop {
		let l = subs.len();
		let w = v.remove(0);
		match w {
			Token::Constant(s)	=> subs.push(s),//subs.push(String::from("__") + &name.clone() + &String::from("__") + &s),
			Token::RParen 		=> break,
			_ 					=> panic!("unknown symbol after function name"),
		};
	};
	// ensure we have at least one substitution -- no variable-free functions!
	assert!(subs.len() > 0);
	
	// collect tokens recursively
	let mut tokens = Vec::<Token>::new();
	let mut paren_count = 0;
	loop {
		let t = v.remove(0);
		match t {
			Token::RParen	=> paren_count -= 1,
			Token::LParen	=> paren_count += 1,
			_				=> (),
		}
		tokens.push(t);
		if paren_count == 0 { break }
	}
	
	// final right paren
	match v.remove(0) { Token::RParen => (), _ => panic!("A function definition must END with a right paren") }
		
	(name, Function {
		substitutions: subs,
		tokens: tokens,
	})
}

pub fn parse_to_variable(mut v: &mut Vec<Token>, environment: &Environment) -> (String,Expression) {
	// all environments start with a lparen
	// environments are non-recursive
	let lparen = v.remove(0);
	match lparen { Token::LParen => (), _ => panic!() }

	// next word must be a "define"
	match v.remove(0) { Token::Keyword(k) => match k { Keyword::Define => {}, _ => panic!() }, _ => panic!(), }

	// next must be another lparen
	match v.remove(0) { Token::LParen => (), _ => panic!() }

	// next will be a name and value
	let name_token = v.remove(0);
	let name = match name_token { Token::Constant(s) => s, _ => panic!() };
    let value = parse(v,&environment);

	// lastly, two rparens
	match v.remove(0) { Token::RParen => (), _ => panic!() }
	match v.remove(0) { Token::RParen => (), _ => panic!() }

	(name,value)
}

pub fn parse_to_environment(mut v: &mut Vec<Token>) -> Environment {

	// a hash to store our environments
    let mut hash: HashMap<String,Expression> = HashMap::new();
	let mut func: HashMap<String,Function> = HashMap::new();
    let mut environment = Environment { variables: hash, functions: func };
	
	// until we have parsed all environment elements
	while v.len() > 0 {

		// determine if we're in a define or deffun
		let def = v.get(1).unwrap().clone();
		match def {
			Token::Keyword(ref k) => match k{
				&Keyword::Define => { let (name,expr) = parse_to_variable(v,&environment); environment.variables.insert(name, expr); },
				&Keyword::Deffun => { let (name,func) = parse_to_function(v);              environment.functions.insert(name, func); },
				_ => panic!("otherwise unknown keyword"), 
			}
			_ => panic!("Must be define or deffun"),
		}
	

 	}
	
	// create the environment and return
    environment
}
