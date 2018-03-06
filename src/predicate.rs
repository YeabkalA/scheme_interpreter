
use lexer::Operator;
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

}
