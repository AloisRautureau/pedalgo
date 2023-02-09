//! contraintes linéaire
use std::ops::Index;
use std::string;

use crate::linear_function::LinearFunction;
use crate::linear_function::Variable;
use crate::{LinearProgram, Simplex};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many_till;

// Variable globale

#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Constraint {
    pub left: LinearFunction,
    pub operator: Operator,
    pub right: LinearFunction,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Constraints {
    inner: Vec<Constraint>,
    nb_var_gap: i32,
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
    ///    left: LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)])),
    ///    operator: Operator::LessEqual,
    ///    right: LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]))
    /// };
    /// let n = Constraint::new(lhs, op, rhs);
    /// assert_eq!(n, expected)
    /// ```
    pub fn new(left: LinearFunction, operator: Operator, right: LinearFunction) -> Constraint {
        Constraint {
            left,
            operator,
            right,
        }
    }

    // Normalizes a constraint with respect to a variable
    pub fn normalize(&self, var: Variable) -> Constraint {
        let (normalized_rhs, coeff) = self.right.normalize(var);
        if coeff != 0.0 {
            Constraint {
                left: -(self.left.clone()) / coeff,
                operator: self.operator.clone(),
                right: normalized_rhs,
            }
        } else {
            self.clone()
        }
    }
}

impl Constraints {
    /// Create a new vector of constraints
    /// # Example
    /// ```rust
    /// ```
    pub fn new() -> Constraints {
        Constraints {
            inner: Vec::new(),
            nb_var_gap: 0,
        }
    }

    pub fn maximize(&self, to_maximize: &LinearFunction) -> Simplex {
        Simplex::from(LinearProgram {
            linear_function: to_maximize.clone(),
            constraints: self.clone(),
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.inner.iter()
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
    /// assert_eq!(constraints[0].operator, Operator::Equal);
    /// assert_eq!(constraints[0].right, LinearFunction::new(-35f32, HashMap::from([(String::from("x"), -32f32), (String::from("y"), 12f32), (String::from("z"), 10f32)])));
    /// ```
    pub fn add_constraint(&mut self, constraint: Constraint) {
        let gap_name = "ε";

        let Constraint {
            left,
            operator,
            right,
        } = constraint;
        match operator {
            Operator::LessEqual | Operator::Less => {
                let x: LinearFunction = LinearFunction::single_variable(
                    gap_name.to_owned() + &self.nb_var_gap.to_string(),
                );
                self.nb_var_gap += 1;

                let constraint = Constraint {
                    left: x,
                    operator: Operator::Equal,
                    right: right - left,
                };
                self.inner.push(constraint);
            }
            Operator::GreaterEqual | Operator::Greater => {
                let x: LinearFunction = LinearFunction::single_variable(
                    gap_name.to_owned() + &self.nb_var_gap.to_string(),
                );
                self.nb_var_gap += 1;

                let constraint = Constraint {
                    left: x,
                    operator: Operator::Equal,
                    right: left - right,
                };
                self.inner.push(constraint);
            }
            Operator::Equal => {
                let x1: LinearFunction = LinearFunction::single_variable(
                    gap_name.to_owned() + &self.nb_var_gap.to_string(),
                );
                self.nb_var_gap += 1;
                let x2: LinearFunction = LinearFunction::single_variable(
                    gap_name.to_owned() + &self.nb_var_gap.to_string(),
                );
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

    // parse a string into a Constraints
    pub fn compile(s: &str) -> Result<Self, ()> {
        let mut constraints = Constraints::default();
        for line in s.lines().filter(|l| !l.trim().is_empty()) {
            constraints.add_constraint(line.parse::<Constraint>()?);
        }
        Ok(constraints)
    }

    /// Normalizes all constraints with respect to a variable
    pub fn normalize(&self, var: Variable) -> Constraints {
        let mut normalized_constraints = self.clone();

        for i in 0..self.inner.len() {
            normalized_constraints.inner[i] = self.inner[i].normalize(var.clone());
        }
        normalized_constraints
    }

    /// Returns the index of the constraints that maximizes 'var' while minimising the corresponding constant
    pub fn constraint_max(&self, var: Variable) -> usize {
        let normalized_constraints = self.normalize(var.clone());

        let mut max_index = 0;
        let mut min_constant = f32::INFINITY;
        let mut current_index = 0;

        for constraint in normalized_constraints.inner.iter() {
            let &index = constraint.right.index(var.clone());
            if (constraint.right.constant < min_constant) && (index < 0.0) {
                min_constant = constraint.right.constant;
                max_index = current_index;
                current_index += 1;
            }
        }

        max_index
    }

    ///
    pub fn pivot_with(&self, var: Variable, i: usize) -> (Constraints, LinearFunction) {
        let mut new_constraints = self.clone();

        let &right_coeff = new_constraints.inner[i].right.index(var.clone());
        let (evar, left_coeff) = new_constraints.inner[i].left.first_positive_coefficient();

        let right_compensation = LinearFunction::single_variable_with_coeff(var, right_coeff);
        let left_compensation = LinearFunction::single_variable_with_coeff(evar, left_coeff);
        new_constraints.inner[i] -= right_compensation;
        new_constraints.inner[i] -= left_compensation;

        let rhs = new_constraints.inner[i].right.clone();
        (new_constraints, rhs)
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
    /// Display a constraint
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
    /// Parses a string into an operator
    /// # Example
    /// ``` rust
    /// use simplex::constraint::Operator;
    /// use std::str::FromStr;
    ///
    /// let operator = match Operator::from_str("<=") {
    ///    Ok(operator) => operator,
    ///    Err(_) => panic!("Error")
    /// };
    /// let expected = Operator::LessEqual;
    /// assert_eq!(operator, expected)
    /// ```
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

    /// Parses a constraint from a string
    /// # Example
    /// ``` rust
    /// use simplex::linear_function::LinearFunction;
    /// use simplex::constraint::Constraint;
    /// use simplex::constraint::Operator;
    /// use std::collections::HashMap;
    /// use std::str::FromStr;
    ///
    /// let constraint = Constraint::from_str("25 -8x + 12 y +3z <= 12")?;
    /// let expected_left = LinearFunction::new(25f32, HashMap::from([(String::from("x"), -8f32), (String::from("y"), 12f32), (String::from("z"), 3f32)]));
    /// let expected_right = LinearFunction::new(12f32, HashMap::new());
    /// let expected = Constraint::new(expected_left, Operator::LessEqual, expected_right);
    /// assert_eq!(constraint, expected)
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_op = alt((
            tag::<&str, &str, ()>("<="),
            tag::<&str, &str, ()>(">="),
            tag::<&str, &str, ()>("="),
            tag::<&str, &str, ()>("<"),
            tag::<&str, &str, ()>(">"),
        ));
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
    /// assert_eq!(c + l_f, expected);
    /// ```
    fn add(self, rhs: LinearFunction) -> Self::Output {
        Constraint {
            left: self.left + rhs.clone(),
            operator: self.operator,
            right: self.right + rhs,
        }
    }
}

impl std::ops::AddAssign<LinearFunction> for Constraint {
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    /// use simplex::constraint::Constraint;
    /// use simplex::constraint::Operator;
    ///
    /// let left = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 15f32), (String::from("y"), -5f32)]));
    /// let right = LinearFunction::new(25f32, HashMap::from([(String::from("x"), -7f32), (String::from("y"), 12f32)]));
    /// let mut c = Constraint::new(left, Operator::Equal, right);
    /// let var_x = LinearFunction::new(-2f32, HashMap::from([(String::from("x"), 5f32)]));
    ///
    /// let expected_left = LinearFunction::new(28f32, HashMap::from([(String::from("x"), 20f32), (String::from("y"), -5f32)]));
    /// let expected_right = LinearFunction::new(23f32, HashMap::from([(String::from("x"), -2f32), (String::from("y"), 12f32)]));
    /// let expected = Constraint::new(expected_left, Operator::Equal, expected_right);
    /// c += var_x;
    /// assert_eq!(c, expected);
    /// ```
    fn add_assign(&mut self, rhs: LinearFunction) {
        self.left += rhs.clone();
        self.right += rhs;
    }
}

impl std::ops::Sub<LinearFunction> for Constraint {
    type Output = Constraint;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let c = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let l_f = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(35f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), -12f32), (String::from("z"), -10f32)]));
    /// assert_eq!(c-l_f, expected)
    /// ```
    fn sub(self, rhs: LinearFunction) -> Self::Output {
        Constraint {
            left: self.left - rhs.clone(),
            operator: self.operator,
            right: self.right - rhs,
        }
    }
}

impl std::ops::SubAssign<LinearFunction> for Constraint {
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    /// use simplex::constraint::Constraint;
    /// use simplex::constraint::Operator;
    ///
    /// let left = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 15f32), (String::from("y"), -5f32)]));
    /// let right = LinearFunction::new(25f32, HashMap::from([(String::from("x"), -7f32), (String::from("y"), 12f32)]));
    /// let mut c = Constraint::new(left, Operator::Equal, right);
    /// let var_x = LinearFunction::new(-2f32, HashMap::from([(String::from("x"), 5f32)]));
    ///
    /// let expected_left = LinearFunction::new(32f32, HashMap::from([(String::from("x"), 10f32), (String::from("y"), -5f32)]));
    /// let expected_right = LinearFunction::new(27f32, HashMap::from([(String::from("x"), -12f32), (String::from("y"), 12f32)]));
    /// let expected = Constraint::new(expected_left, Operator::Equal, expected_right);
    /// c -= var_x;
    /// assert_eq!(c, expected);
    /// ```
    fn sub_assign(&mut self, rhs: LinearFunction) {
        self.left -= rhs.clone();
        self.right -= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_normalize() {
        let constraints =
            Constraints::compile("x + 2y >= 0 \n2 + x - 2y <= 4").unwrap();
        let normalize_constraints = constraints.normalize("x".to_string());

        let expected = Constraints::compile("ε0 = x + 2y\nε1 = 2 - x + 2y").unwrap();
        assert_eq!(normalize_constraints, expected);
    }
    */

    fn test_sub_assign_constraint() {
        use std::collections::HashMap;
        use crate::constraint::Constraint;
        use crate::constraint::Operator;
        use std::str::FromStr;
        
        let mut c = Constraint::from_str("0 = 200 - x - y").unwrap();
        let l_f = LinearFunction::new(0f32, HashMap::from([(String::from("x"), -1f32)]));
    
        let expected =  Constraint::from_str("x = 200 - y + 0x").unwrap();
        c -= l_f;
        assert_eq!(c, expected);
    }
}
