use super::expression::Expression;


// Add{Multiply{Number{2},Number{3}},Number{4}}
pub trait Operator {
    fn try_new(args: Vec<Expression>) -> anyhow::Result<Self> where Self: Sized;
    fn reduce(self) -> anyhow::Result<Expression>;
    fn is_reducible(&self) -> bool;
    fn to_string(&self) -> String;
}