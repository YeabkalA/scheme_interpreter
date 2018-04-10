
use lexer::Operator;
use lexer::CompoundPredicateType;
use evaluator::Expression;
use evaluator::evaluate;

#[derive(Clone)]
#[derive(Debug)]
pub struct Predicate {
    pub operator: Operator,
    pub l_hand: Expression,
    pub r_hand: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

impl Predicate {

    pub fn evaluate(&self) -> Expression {
        let l_val = evaluate(&self.l_hand).ok().unwrap();
        let r_val = evaluate(&self.r_hand).ok().unwrap();
        match self.operator {
            Operator::Greater => if l_val > r_val  { return self.if_true.clone() } else { return self.if_false.clone() },
            Operator::Less    => if l_val < r_val  { return self.if_true.clone() } else { return self.if_false.clone() },
            Operator::Equal   => if l_val == r_val  { return self.if_true.clone() } else { return self.if_false.clone() },
            _                 => panic!("unexpected operator in predicate"),
        }
    }
	pub fn evaluate_no_expression(&self) -> bool {
        let l_val = evaluate(&self.l_hand).ok().unwrap();
        let r_val = evaluate(&self.r_hand).ok().unwrap();
        match self.operator {
            Operator::Greater => if l_val > r_val  { return true } else {},
            Operator::Less    => if l_val < r_val  { return true } else {},
            Operator::Equal   => if l_val == r_val  { return true } else {},
            _                 => panic!("unexpected operator in predicate"),
        };
		return false;
	}

}

#[derive(Clone)]
#[derive(Debug)]
pub struct CompoundPredicate {
	pub compound: CompoundPredicateType,
	pub list: Vec<Predicate>,
	pub if_true: Expression,
	pub if_false: Expression,
}

impl CompoundPredicate {

	fn count_true_predicates(&self) -> usize {
		return self.list.iter().fold(0, |acc,f| if f.evaluate_no_expression() { acc + 1 } else { acc } )
	}

	fn evaluate_or(&self) -> Expression {
		let count = self.count_true_predicates();
		if count > 0 { return self.if_true.clone() }
		self.if_false.clone()
	}

	fn evaluate_and(&self) -> Expression {
		let count = self.count_true_predicates();
		if count == self.list.len() { return self.if_true.clone() }
		self.if_false.clone()
	}

	fn evaluate_not(&self) -> Expression {
		let count = self.count_true_predicates();
		if count == 0 { return self.if_true.clone() }
		self.if_false.clone()
	}

	pub fn evaluate(&self) -> Expression {
		match self.compound {
			CompoundPredicateType::Or		=> self.evaluate_or(),
			CompoundPredicateType::And		=> self.evaluate_and(),
			CompoundPredicateType::Not		=> self.evaluate_not(),
		}
	}
	
}