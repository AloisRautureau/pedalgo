//! contraintes linéaire
use crate::linear_function::LinearFunction;
use crate::linear_function::Variable;

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

impl Operator {
    /// ```rust
    /// let a = Operator::Less;
    /// let b = Operator::GreaterEqual;
    /// assert_eq!(a.inverse(), b)
    ///
    /// let c = Operator::Greater;
    /// let d = Operator::LessEqual;
    /// assert_eq!(c.inverse(), d)
    /// ```
    pub fn inverse(&self) -> Operator {
        match self {
            Operator::Equal => Operator::Equal,
            Operator::Less => Operator::GreaterEqual,
            Operator::Greater => Operator::LessEqual,
            Operator::LessEqual => Operator::Greater,
            Operator::GreaterEqual => Operator::Less,
        }
    }
}

impl Constraint {
    /// Create a new constraint with a left linear function, an operator and a right linear function
    /// always with the form
    ///     - [Zero] < [LinearFunction]
    ///     - [Zero] <= [LinearFunction]
    ///     - [Zero] =  [LinearFunction]
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let lhs = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let rhs = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let op = Operator::LessEqual;
    /// let expected = Constraint {
    ///    left: LinearFunction::zero(),
    ///    operator: Operator::LessEqual,
    ///    right: LinearFunction::new(-35f32, HashMap::from([(String::from("x"), -32f32), (String::from("y"), 12f32), (String::from("z"), 10f32)]))
    /// };
    /// assert_eq!(new(lhs,op,rhs), expected)
    /// ```
    pub fn new(left: LinearFunction, operator: Operator, right: LinearFunction) -> Constraint {
        match operator {
            Operator::Less | Operator::LessEqual | Operator::Equal => Constraint {
                left: LinearFunction::zero(),
                operator: operator,
                right: right - left,
            },
            Operator::Greater | Operator::GreaterEqual => Constraint {
                left: LinearFunction::zero(),
                operator: operator.inverse(),
                right: left - right,
            },
        }
    }

	// Normalizes a constraint with respect to a variable
	pub fn normalize(&self, var: Variable) -> Constraint {
		let (normalized_rhs, coeff) = self.right.normalize(var.clone());

		Constraint {
			left: -(self.left.clone()) / coeff,
			operator: self.operator.clone(),
			right: normalized_rhs,
		}
	}
}

impl Constraints {
    pub fn add_constraint(&mut self, constraint: Constraint) {
        match constraint.operator {
            Operator::Less => {
                let constraint1 = Constraint {
                    left: LinearFunction::zero(),
                    operator: Operator::LessEqual,
                    right: LinearFunction::zero(),
                };
                self.inner.push(constraint1);
            }
            Operator::Greater => {
                let constraint1 = Constraint {
                    left: LinearFunction::zero(),
                    operator: Operator::LessEqual,
                    right: LinearFunction::zero(),
                };
                self.inner.push(constraint1);
            }
            _ => {
                let constraint1 = Constraint {
                    left: LinearFunction::zero(),
                    operator: Operator::LessEqual,
                    right: LinearFunction::zero(),
                };
                self.inner.push(constraint1);
            }
        }
    }

	pub fn normalize(&self, var: Variable) {}
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
            writeln!(f, "{constraint}")?;
        }
        Ok(())
    }
}

/*
OPERATOR OVERLOADING
 */
impl std::ops::Add<LinearFunction> for Constraint {
    type Output = Constraint;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let c = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let l_f = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(25f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), 12f32), (String::from("z"), 0f32)]));
    /// assert_eq!(a + b, expected)
    /// ```
    fn add(self, rhs: LinearFunction) -> Self::Output {
        Constraint {
            left: self.left + rhs.clone(),
            operator: self.operator,
            right: self.right + rhs,
        }
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
