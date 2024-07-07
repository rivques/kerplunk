use anyhow::Context;

use crate::core::{errors::{NonreduceibleExpressionError, WrongNumberOfArgumentsError}, expression::{Element, Expression}, operator::{reduce_ops_then_run_if_numeric, Operator, ReductionResult}};

pub struct Multiply {
    args: Vec<Expression>,
}

impl Multiply {
    
}

impl Operator for Multiply {
    fn try_new(mut args: Vec<Expression>) -> anyhow::Result<Self> {
        if args.len() != 2 {
            return Err(WrongNumberOfArgumentsError{expected: 2, actual: args.len()}.into());
        }
        // this should never error, because of the above check
        Ok(Multiply {
            args,
        })
    }
    // is there any Operator that _doesn't_ follow the "reduce args then eval smth if args are all numbers" pattern?
    // yes, Integrate
    // dammit
    // okay so i want to get rid of what is likely to be boilerplate
    // this check-args-and-reduce-if-possible pattern is likely to be common
    // but _sometimes_ i want to do something else
    // so how can i optionally provide that?
    // Operator::reduce_ops_then_run_if_numeric(args: Vec<Expression>, theFn: fn(Vec<f64>) -> f64) -> ReductionResult

    fn reduce(&mut self) -> ReductionResult {
        // associativity and commutativity: how??
        // so multiplication is associative and commutative with itself, and is associative with division
        // 3*(5*2) = 2*(5*3)
        // (3*2)/4 = 3*(2/4)
        // this seems like i need to solve this problem elsewhere, in a more general way

        // anyway, for now, i'll just reduce the args and multiply them
        return reduce_ops_then_run_if_numeric(&mut self.args, |args| args[0] * args[1])
    }

    fn to_string(&self) -> String {
        format!("Multiply{{{},{}}}", self.args[0].to_string(), self.args[1].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::expression::Element;

    #[test]
    fn multiply_numbers() {
        let result = Multiply::try_new(vec![
            Expression::new(Element::Number(2.0)),
            Expression::new(Element::Number(3.0)),
        ]).unwrap().reduce();
        if let ReductionResult::ReducedToNumber(result) = result {
            assert_eq!(result, 6.0);
        } else {
            panic!("Expected Number, got {:?}", result);
        }
    }

    #[test]
    fn multiply_variable_and_number() {
        let mut op = Multiply::try_new(vec![
            Expression::new(Element::Variable("x".to_string())),
            Expression::new(Element::Number(3.0)),
        ]).unwrap();
        let result = op.reduce();
        assert_eq!(result, ReductionResult::StayedInPlace);
        assert_eq!(op.to_string(), "Multiply{Variable{x},Number{3}}");
    }

    #[test]
    fn multiply_nested_numbers() {
        let result = Multiply::try_new(vec![
            Expression::new(Element::Number(2.0)),
            Expression::new(Element::Operator(Box::new(Multiply::try_new(vec![
                Expression::new(Element::Number(3.0)),
                Expression::new(Element::Number(4.0)),
            ]).unwrap()))),
        ]).unwrap().reduce();
        if let ReductionResult::ReducedToNumber(result) = result {
            assert_eq!(result, 24.0);
        } else {
            panic!("Expected Number, got {:?}", result);
        }
    }

    #[test]
    fn multiply_nested_combo() {
        let mut op = Multiply::try_new(vec![
            Expression::new(Element::Variable("x".to_string())),
            Expression::new(Element::Operator(Box::new(Multiply::try_new(vec![
                Expression::new(Element::Number(3.0)),
                Expression::new(Element::Number(4.0)),
            ]).unwrap()))),
        ]).unwrap();
        let result = op.reduce();
        assert_eq!(result, ReductionResult::StayedInPlace);
        assert_eq!(op.to_string(), "Multiply{Variable{x},Number{12}}");
    }

    // TODO: get associativity and commutativity working

    // #[test]
    // fn multiply_nested_associative() {
    //     let mut op = Multiply::try_new(vec![
    //         Expression::new(Element::Number(4.0)),
    //         Expression::new(Element::Operator(Box::new(Multiply::try_new(vec![
    //             Expression::new(Element::Number(3.0)),
    //             Expression::new(Element::Variable("x".to_string())),
    //         ]).unwrap()))),
    //     ]).unwrap();
    //     let result = op.reduce();
    //     assert_eq!(result, ReductionResult::StayedInPlace);
    //     assert_eq!(op.to_string(), "Multiply{Number{12},Variable{x}}");
    // }
}