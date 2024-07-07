use anyhow::Context;

use crate::core::{errors::WrongNumberOfArgumentsError, expression::{Element, Expression}, operator::Operator};

pub struct Multiply {
    arg1: Expression,
    arg2: Expression,
}

impl Operator for Multiply {
    fn try_new(mut args: Vec<Expression>) -> anyhow::Result<Self> {
        if args.len() != 2 {
            return Err(WrongNumberOfArgumentsError{expected: 2, actual: args.len()}.into());
        }
        // this should never error, because of the above check
        Ok(Multiply {
            arg2: args.pop().context("Failed to pop arg2, somehow")?,
            arg1: args.pop().context("Failed to pop arg1, somehow")?, // note that the order of arguments is reversed because we're popping them off the end
        })
    }

    fn reduce(mut self) -> anyhow::Result<Expression> {
        if !self.is_reducible(){
            return Ok(Expression::new(Element::Operator(Box::new(self))))
        }
        // if there's a reducible operator, reduce it
        if let Element::Operator(op) = self.arg1.element {
            if op.is_reducible() {
                self.arg1 = op.reduce()?;
            }
        } else {}
        if let Element::Operator(op) = self.arg2.element {
            if op.is_reducible() {
                self.arg2 = op.reduce()?;
            }
        }

        // now, see what we're made of
        // if we're two numbers, multiply them
        if let Element::Number(num1) = &self.arg1.element {
            if let Element::Number(num2) = &self.arg2.element {
                return Ok(Expression::new(Element::Number(num1 * num2)));
            }
        }
        // otherwise, return ourselves
        Ok(Expression::new(Element::Operator(Box::new(self))))
    }

    fn is_reducible(&self) -> bool {
        // {1, 3*2} is reducible
        // {1, 3} is reducible
        // {3, x} is not reducible
        // {4, {3, x}} is not reducible
        // what's the pattern?
        // a reducible operator is always reducible. if there's no reducible operator:
        // it's only reducible if both arguments are numbers

        // if there's a reducible operator, return true
        if let Element::Operator(op) = &self.arg1.element {
            if op.is_reducible() {
                return true;
            }
        }
        if let Element::Operator(op) = &self.arg2.element {
            if op.is_reducible() {
                return true;
            }
        }

        // if there's no reducible operator, return true if both arguments are numbers
        if let Element::Number(_) = self.arg1.element {
            if let Element::Number(_) = self.arg2.element {
                return true;
            }
        }

        false
    }

    fn to_string(&self) -> String {
        format!("Multiply{{{},{}}}", self.arg1.to_string(), self.arg2.to_string())
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
        ]).unwrap().reduce().unwrap();
        if let Element::Number(result) = result.element {
            assert_eq!(result, 6.0);
        } else {
            panic!("Expected Number, got {result}");
        }
    }

    #[test]
    fn multiply_variable_and_number() {
        let result = Multiply::try_new(vec![
            Expression::new(Element::Variable("x".to_string())),
            Expression::new(Element::Number(3.0)),
        ]).unwrap().reduce().unwrap();
        if let Element::Operator(_) = result.element {
            // this is the expected result
        } else {
            panic!("Expected Operator, got {result}");
        }
    }

    #[test]
    fn reducibility_num_and_num() {
        let operator = Multiply::try_new(vec![
            Expression::new(Element::Number(2.0)),
            Expression::new(Element::Number(3.0)),
        ]).unwrap();
        assert!(operator.is_reducible());
    }

    #[test]
    fn reducibility_num_and_var() {
        let operator = Multiply::try_new(vec![
            Expression::new(Element::Number(2.0)),
            Expression::new(Element::Variable("x".to_string())),
        ]).unwrap();
        assert!(!operator.is_reducible());
    }

    #[test]
    fn reducibility_var_and_var() {
        let operator = Multiply::try_new(vec![
            Expression::new(Element::Variable("x".to_string())),
            Expression::new(Element::Variable("y".to_string())),
        ]).unwrap();
        assert!(!operator.is_reducible());
    }

    #[test]
    fn reducibility_multnumnum_and_num() {
        let operator = Multiply::try_new(vec![
            Expression::new(Element::Operator(Box::new(Multiply::try_new(vec![
                Expression::new(Element::Number(2.0)),
                Expression::new(Element::Number(3.0)),
            ]).unwrap()))),
            Expression::new(Element::Number(4.0)),
        ]).unwrap();
        assert!(operator.is_reducible());
    }

    #[test]
    fn reducibility_multnumnum_and_var() {
        let operator = Multiply::try_new(vec![
            Expression::new(Element::Operator(Box::new(Multiply::try_new(vec![
                Expression::new(Element::Number(2.0)),
                Expression::new(Element::Number(3.0)),
            ]).unwrap()))),
            Expression::new(Element::Variable("x".to_string())),
        ]).unwrap();
        assert!(operator.is_reducible());
    }
}