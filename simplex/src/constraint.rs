//! contraintes linéaire
use crate::linear_function::LinearFunction;
use crate::linear_function::Variable;
use crate::{LinearProgram, Simplex};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many_till;

// Variable globale

#[derive(Debug, Clone, Default)]
pub enum Operator {
    #[default]
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

/// A Constraint is a linear function with an operator
/// [linear_function] [operator] [0]
#[derive(Debug, Clone, Default)]
pub struct Constraint {
    pub left: LinearFunction,
    pub operator: Operator,
    pub right: LinearFunction,
}

#[derive(Debug, Clone, Default)]
pub struct Constraints {
    inner: Vec<Constraint>,
    nb_var_gap: i32,
}
impl Constraints {
    pub fn maximize(&self, to_maximize: &LinearFunction) -> Simplex {
        Simplex::from(LinearProgram {
            linear_function: to_maximize.clone(),
            constraints: self.clone(),
        })
    }

    pub fn minimize(&self, to_minimize: &LinearFunction) -> Simplex {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.inner.iter()
    }
}

impl Operator {
    /// ```rust
    /// use simplex::constraint::Operator;
    /// let a = Operator::Less;
    /// let b = Operator::GreaterEqual;
    /// assert_eq!(a.inverse(), b);
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
    /// Create a new constraint from two linear functions and an operator
    /// [left::LinearFunction] [op::Operator] [right::LinearFunction]
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::constraint::{Constraint, Operator};
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
                operator,
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
        let (normalized_rhs, coeff) = self.right.normalize(var);

        Constraint {
            left: -(self.left.clone()) / coeff,
            operator: self.operator.clone(),
            right: normalized_rhs,
        }
    }
}

impl Constraints {
    /// Create a new vector of constraints
    /// # Example
    /// ```rust
    /// use simplex::constraint::Constraints;
    ///
    /// let constraints = Constraints::new();
    /// ```
    pub fn new() -> Constraints {
        Constraints {
            inner: Vec::new(),
            nb_var_gap: 0,
        }
    }

    /// Add a constraint to the list of constraints
    /// The constraint added is in this form :
    ///
    /// [Gap_Variable] [=] [Constant] + [LinearFunction_of_non_gap_variables]
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::constraint::{Constraint, Constraints, Operator};
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let mut constraints = Constraints::new();
    /// let constraint = Constraint {
    ///   left: LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)])),
    ///   operator: Operator::LessEqual,
    ///   right: LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]))
    /// };
    /// constraints.add_constraint(constraint);
    /// assert_eq!(constraints.gap_variables_count(), 1);
    /// assert_eq!(constraints[0].left, LinearFunction::zero());
    /// assert_eq!(constraints[0].operator, Operator::Equal);
    /// assert_eq!(constraints[0].right, LinearFunction::new(-35f32, HashMap::from([(String::from("x"), -32f32), (String::from("y"), 12f32), (String::from("z"), 10f32)])));
    /// ```
    pub fn add_constraint(&mut self, constraint: Constraint) {
        let Constraint {
            left,
            operator,
            right,
        } = constraint;
        match operator {
            Operator::LessEqual | Operator::Less => {
                let x: LinearFunction =
                    LinearFunction::single_variable("E".to_owned() + &self.nb_var_gap.to_string());
                self.nb_var_gap += 1;

                let constraint = Constraint {
                    left: x,
                    operator: Operator::Equal,
                    right: right - left,
                };
                self.inner.push(constraint);
            }
            Operator::GreaterEqual | Operator::Greater => {
                let x: LinearFunction =
                    LinearFunction::single_variable("E".to_owned() + &self.nb_var_gap.to_string());
                self.nb_var_gap += 1;

                let constraint = Constraint {
                    left: x,
                    operator: Operator::Equal,
                    right: left - right,
                };
                self.inner.push(constraint);
            }
            Operator::Equal => {
                let x1: LinearFunction =
                    LinearFunction::single_variable("E".to_owned() + &self.nb_var_gap.to_string());
                self.nb_var_gap += 1;
                let x2: LinearFunction =
                    LinearFunction::single_variable("E".to_owned() + &self.nb_var_gap.to_string());
                self.nb_var_gap += 1;

                let constraint1 = Constraint {
                    left: x1,
                    operator: Operator::Equal,
                    right: right.clone() - left.clone(),
                };
                let constraint2 = Constraint {
                    left: x2,
                    operator: Operator::Equal,
                    right: right - left,
                };
                self.inner.push(constraint1);
                self.inner.push(constraint2);
            }
        }
    }

    pub fn gap_variables_count(&self) -> usize {
        self.inner.len()
    }

    pub fn normalize(&self, var: Variable) {
        todo!()
    }
}

impl std::ops::Index<usize> for Constraints {
    type Output = Constraint;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
impl std::ops::IndexMut<usize> for Constraints {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
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
            writeln!(f, "{constraint}")?;
        }
        Ok(())
    }
}

/*
PARSING
 */
impl std::str::FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "=" => Ok(Operator::Equal),
            "<" => Ok(Operator::Less),
            ">" => Ok(Operator::Greater),
            "<=" => Ok(Operator::LessEqual),
            ">=" => Ok(Operator::GreaterEqual),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for Constraint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_op = alt((
            tag::<&str, &str, ()>("<="),
            tag::<&str, &str, ()>(">="),
            tag::<&str, &str, ()>("="),
            tag::<&str, &str, ()>("<"),
            tag::<&str, &str, ()>(">"),
        ));
        println!("{s}");
        if let Ok((rhs, (lhs, op))) = many_till(anychar, parse_op)(s) {
            let lhs = lhs
                .iter()
                .fold(String::new(), |acc, c| acc + &c.to_string());
            Ok(Constraint::new(
                lhs.parse::<LinearFunction>()?,
                op.parse()?,
                rhs.parse::<LinearFunction>()?,
            ))
        } else {
            Err(())
        }
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
    /// assert_eq!(c + l_f, expected)
    /// ```
    fn add(self, rhs: LinearFunction) -> Self::Output {
        Constraint {
            left: self.left + rhs.clone(),
            operator: self.operator,
            right: self.right + rhs,
        }
    }
}

/*
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


------------------            _____
max x + 3y;      |           | RUN |
2x - 5y <= 10;   |            -----
x + y <= 5;      |
x <= 0;          |
-----------------

ET( linear_function, OP(l_f, operor, l_f),
*/
