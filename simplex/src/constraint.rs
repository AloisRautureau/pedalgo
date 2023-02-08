//! contraintes linéaire
use std::collections::HashMap;
use crate::linear_function::LinearFunction;

/// Contraintes object

#[derive(Debug, Clone)]
pub enum Operator {
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

/// A Constraint is a linear function with an operator
/// [linear_function] [operator] [0]
#[derive(Debug, Clone)]
pub struct Constraint {
    pub left: LinearFunction,
    pub operator: Operator,
    pub right: LinearFunction,
}

#[derive(Debug, Clone)]
pub struct Constraints {
    inner: Vec<Constraint>,
}



impl Constraint {
    pub fn new(left: LinearFunction, operator: Operator, right: LinearFunction) -> Constraint {
        match operator {
            Operator::Equal => Constraint {
                left: left - right ,
                operator: operator,
                right: LinearFunction{constant: 0.0, coefficients: HashMap::new()},
            },
            Operator::Less => Constraint {
                left: left,
                operator: operator,
                right: right,
            },
            Operator::Greater => Constraint {
                left: left,
                operator: operator,
                right: right,
            },
            Operator::LessEqual => Constraint {
                left: -left,
                operator: operator,
                right: right,
            },
            Operator::GreaterEqual => Constraint {
                left,
                operator,
                right,
            },
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::Less => write!(f, "<"),
            Operator::Greater => write!(f, ">"),
            Operator::LessEqual => write!(f, "<="),
            Operator::GreaterEqual => write!(f, ">="),
        }
    }
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl std::fmt::Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for constraint in self.inner.iter() {
            writeln!(f, "{}", constraint)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // let left = LinearFunction::new(vec![1.0, 2.0, 3.0]);
        // let right = LinearFunction::new(vec![1.0, 2.0, 3.0]);
        // let constraint = Constraint::new(left, Operator::Equal, right);
        // assert_eq!(constraint.left, LinearFunction::new(vec![0.0, 0.0, 0.0]));
        todo!();
    }
}

/*
------------------            _____
max x + 3y;      |           | RUN |
2x - 5y <= 10;   |            -----
x + y <= 5;      |
x <= 0;          |
-----------------

ET( linear_function, OP(l_f, operor, l_f),
*/
