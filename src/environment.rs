
use evaluator::*;
use function::*;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
pub struct Environment {

    pub variables: HashMap<String,Expression>,
	pub functions: HashMap<String,Function>,

}


