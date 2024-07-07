use super::expression::{Element, Expression};


// Add{Multiply{Number{2},Number{3}},Number{4}}
pub trait Operator {
    fn try_new(args: Vec<Expression>) -> anyhow::Result<Self> where Self: Sized;
    fn reduce(&mut self) -> ReductionResult;
    fn to_string(&self) -> String;
}

pub fn reduce_ops_then_run_if_numeric(args: &mut Vec<Expression>, the_fn: fn(Vec<f64>) -> f64) -> ReductionResult {
    let mut all_numbers: Vec<f64> = vec![];
    let mut all_reduced = true;
    for arg in args {
        match &mut arg.element {
            Element::Number(num) => all_numbers.push(*num),
            Element::Operator(op) => {
                match op.reduce() {
                    ReductionResult::ReducedToNumber(num) => {
                        all_numbers.push(num);
                        arg.element = Element::Number(num);
                    },
                    ReductionResult::StayedInPlace => all_reduced = false,
                }
            },
            Element::Variable(_) => all_reduced = false,
        }
    }

    if all_reduced {
        let result = the_fn(all_numbers);
        ReductionResult::ReducedToNumber(result)
    } else {
        ReductionResult::StayedInPlace
    }
}

#[derive(Debug, PartialEq)]
pub enum ReductionResult {
    StayedInPlace, // when either nothing changed or a component was reduced, but this operator was not totally evaluated
    ReducedToNumber(f64), // when the operator was reduced to a number
}