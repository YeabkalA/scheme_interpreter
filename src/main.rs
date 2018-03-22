use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate updated_scheme;

use updated_scheme::interpreter::*;
use updated_scheme::environment::*;
use updated_scheme::lexer::*;
use updated_scheme::function::*;
use updated_scheme::evaluator::*;
use updated_scheme::environment_parser::*;
use updated_scheme::parser::*;

fn parse_and_evaluate(tokens: &mut Vec::<Token>, env: &Environment) {
    let expression = parse(tokens, &env);
    let result = evaluate(&expression);
	println!("{}", result.ok().unwrap());
}

fn parse_env(tokens: &mut Vec::<Token>, env: &mut Environment) {
	let new_env = parse_to_environment(tokens);
	env.variables = env.variables.clone().into_iter().chain(new_env.variables).collect();
	env.functions = env.functions.clone().into_iter().chain(new_env.functions).collect();
}

fn main() {
	
    let mut hash: HashMap<String,Expression> = HashMap::new();
	let mut func: HashMap<String,Function> = HashMap::new();
    let mut environment = Environment { variables: hash, functions: func };
	
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
		let line = line.unwrap();
		let mut tokens = tokenize(&line);
		let two = tokens.get(1).unwrap().clone();
		match two {
			Token::Keyword(ref k) => {
				match k {
					&Keyword::Define => {parse_env(&mut tokens, &mut environment)},
					&Keyword::Deffun => {parse_env(&mut tokens, &mut environment)},
					_ => parse_and_evaluate(&mut tokens, &environment),
				}
			},
			_ => parse_and_evaluate(&mut tokens, &environment)
		};
    }
}
