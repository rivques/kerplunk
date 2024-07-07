// like 1, x, 4y^2, 3/ms^2, velocity m/s^2

use std::fmt::Display;

use super::{operator::Operator};
use anyhow::{Result, anyhow};

pub struct Expression {
    pub element: Element,
}


pub enum Element {
    Operator(Box<dyn Operator>),
    Number(f64),
    Variable(String),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Operator(op) => write!(f, "{}", op.to_string()),
            Element::Number(num) => write!(f, "Number{{{}}}", num),
            Element::Variable(var) => write!(f, "Variable{{{}}}", var),
        }
    }
}

impl Expression {
    pub fn new(element: Element) -> Self {
        Expression { element }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element.to_string())
    }
}