use evaluator::*;
use lexer::*;
use parser::*;
use environment::*;

#[derive(Clone)]
#[derive(Debug)]
pub struct Function {
	pub substitutions: Vec<String>,
	pub tokens: Vec<Token>,
}

impl Function {

	// An environment is provided
    pub fn parse(&self, args: Vec<Expression>, env: &Environment) -> Expression {
		assert_eq!(self.substitutions.len(), args.len(), "incorrect number of arguments");

		// add all of the substitutions into our temporary environment, which we will
		// use for parsing.
		let mut temporary_environment = env.clone();
		for i in 0..args.len() {
			temporary_environment.variables.insert(self.substitutions[i].clone(), args[i].clone());
		};
		
		// parse the tokens and return as an expression
		let mut tok = self.tokens.clone();
		parse(&mut tok,&temporary_environment)
	}
	
}
