//! contraintes linéaire
use crate::linear_function::LinearFunction;
use crate::linear_function::Variable;
use crate::linear_function::GAP_VARIABLE_IDENTIFIER;
use crate::{LinearProgram, Simplex};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many_till;

// Variable globale

#[derive(Debug, Clone, Default, PartialEq, Copy)]
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
}

impl Operator {
    /// ```rust
    /// use simplex::constraint::Operator;
    /// let a = Operator::Less;
    /// let b = Operator::GreaterEqual;
    /// assert_eq!(a.inverse(), b);
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
    /// let lhs = LinearFunction::new(0f32, HashMap::from([(String::from("x"), 32f32)]));
    /// let rhs = LinearFunction::new(0f32, HashMap::new());
    /// let op = Operator::LessEqual;
    /// let expected = Constraint {
    ///    left: LinearFunction::new(0f32, HashMap::from([(String::from("x"), 32f32)])),
    ///    operator: Operator::LessEqual,
    ///    right: LinearFunction::new(0f32, HashMap::new()),
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
    pub fn normalize(&mut self, var: &Variable) {
        if self.right.contains(var) {
            self.left /= self.right[var];
            self.right /= self.right[var];
        }
    }

    pub fn is_valid_linear_programm(&self) -> bool {
        self.left.is_one_normalized_var() && self.operator == Operator::Equal
    }

    pub fn non_gap_variables(&self) -> Vec<Variable> {
        union(
            self.left.non_gap_variables(),
            self.right.non_gap_variables(),
        )
    }
}

impl Constraints {
    /// Create a new vector of constraints
    pub fn new() -> Constraints {
        Constraints { inner: Vec::new() }
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
        let next_gap_var = || {
            LinearFunction::single_variable(format!(
                "{GAP_VARIABLE_IDENTIFIER}{}",
                self.gap_variables_count()
            ))
        };

        let Constraint {
            left,
            operator,
            right,
        } = constraint;
        match operator {
            Operator::LessEqual | Operator::Less => {
                let constraint = Constraint {
                    left: next_gap_var(),
                    operator: Operator::Equal,
                    right: right - left,
                };
                self.inner.push(constraint);
            }
            Operator::GreaterEqual | Operator::Greater => {
                let constraint = Constraint {
                    left: next_gap_var(),
                    operator: Operator::Equal,
                    right: left - right,
                };
                self.inner.push(constraint);
            }
            Operator::Equal => {
                let constraint1 = Constraint {
                    left: next_gap_var(),
                    operator: Operator::Equal,
                    right: right.clone() - left.clone(),
                };
                let constraint2 = Constraint {
                    left: next_gap_var(),
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
    pub fn normalize(&mut self, var: &Variable) {
        self.inner.iter_mut().for_each(|c| c.normalize(var))
    }

    /// Returns the index of the constraint that maximizes 'var' while minimising the corresponding constant
    pub fn most_restrictive(&self, var: &Variable) -> Option<usize> {
        self.iter()
            .enumerate()
            .filter(|(_, c)| c.right.contains(var) && c.right[var] <= 0.0)
            .max_by(
                |(_, Constraint { right: a, .. }), (_, Constraint { right: b, .. })| {
                    let restriction_a = a.constant / a[var];
                    let restriction_b = b.constant / b[var];
                    restriction_a.total_cmp(&restriction_b)
                },
            )
            .map(|(i, _)| i)
    }

    /// Performs a pivot step on a particular constraint with respect to a specific variable
    pub fn pivot(&mut self, constraint_index: usize, var: &Variable) {
        // Pivot the particular constraint we've targeted
        {
            let constraint = &mut self.inner[constraint_index];
            constraint.normalize(var);
            *constraint -= constraint.left.clone();
            *constraint -= LinearFunction::single_variable(var.to_string());
            *constraint = -constraint.clone();
        }
        // And replace the variable by the new rhs in other constraints
        let func = self.inner[constraint_index].right.clone();
        self.replace_variable_with(var, &func);
    }

    pub fn is_valid(&self) -> bool {
        for constraint in self.inner.iter() {
            if !constraint.is_valid_linear_programm() {
                return false;
            }
        }
        true
    }
    pub fn non_gap_variables(&self) -> Vec<Variable> {
        let mut variables = Vec::new();
        for constraint in self.inner.iter() {
            variables = union(variables, constraint.non_gap_variables());
        }
        variables
    }

    fn replace_variable_with(&mut self, var: &Variable, value: &LinearFunction) {
        for Constraint { right, .. } in &mut self.inner {
            right.replace(var, value)
        }
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

pub fn union<T: Clone + PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut res = a.clone();
    for elem in b {
        if !a.contains(&elem) {
            res.push(elem);
        }
    }
    res
}

pub fn is_nearly_equal(a:Vec<f32>, b: Vec<f32>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if (a[i] - b[i]).abs() > 0.001 {
            return false;
        }
    }
    true
}

// Normalize all point in a vector
// choose the factor of normalization to be the maximum of the absolute value of the coordinates
pub fn normalized_vec(a: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut res:Vec<Vec<f32>> = Vec::new();
    let mut max = 0.0;
    for point in a.clone().iter() {
        for coord in point.iter() {
            if coord.abs() > max {
                max = coord.abs();
            }
        }
    }
    for point in a.iter() {
        let mut normalized_point = Vec::new();
        for coord in point.iter() {
            normalized_point.push(coord / max);
        }
        res.push(normalized_point);
    }
    res 
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
    /// let constraint = Constraint::from_str("25 -8x + 12y +3z <= 12").unwrap();
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
    ///
    /// c -= var_x;
    /// assert_eq!(c, expected);
    /// ```
    fn sub_assign(&mut self, rhs: LinearFunction) {
        self.left -= rhs.clone();
        self.right -= rhs;
    }
}

impl std::ops::Div<f32> for Constraint {
    type Output = Constraint;

    fn div(self, rhs: f32) -> Self::Output {
        Constraint {
            left: self.left / rhs,
            operator: self.operator,
            right: self.right / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Constraint {
    fn div_assign(&mut self, rhs: f32) {
        self.left /= rhs;
        self.right /= rhs;
    }
}

impl std::ops::Neg for Constraint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            left: -self.left,
            right: -self.right,
            operator: self.operator.inverse(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_operator() {
        let c = Operator::Greater;
        let d = Operator::LessEqual;
        assert_eq!(c.inverse(), d)
    }

    #[test]
    fn test_new_constrait() {
        use std::collections::HashMap;

        let lhs = LinearFunction::new(
            30f32,
            HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]),
        );
        let rhs = LinearFunction::new(
            -5f32,
            HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]),
        );
        let op = Operator::LessEqual;
        let expected = Constraint {
            left: LinearFunction::new(
                30f32,
                HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]),
            ),
            operator: Operator::LessEqual,
            right: LinearFunction::new(
                -5f32,
                HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]),
            ),
        };
        let n = Constraint::new(lhs, op, rhs);
        assert_eq!(n, expected)
    }

    #[test]
    fn test_normalize() {
        let mut constraints =
            Constraints::compile("x - 2y >= 6 \n 12 + 9x + 3y <= 6\n 1 + 7x - y <= 0").unwrap();
        constraints.normalize(&"y".to_string());

        assert_eq!(constraints.inner[0].right[&"y".to_string()], 1.0);
        assert_eq!(constraints.inner[1].right[&"y".to_string()], 1.0);
        assert_eq!(constraints.inner[2].right[&"y".to_string()], 1.0);
    }

    #[test]
    fn test_sub_assign_constraint() {
        use std::collections::HashMap;
        use std::str::FromStr;

        let mut c = Constraint::from_str("0 = 200 - x - y").unwrap();
        let l_f = LinearFunction::new(0f32, HashMap::from([(String::from("x"), -1f32)]));

        let expected = Constraint::from_str("x = 200 - y + 0x").unwrap();
        c -= l_f;
        assert_eq!(c, expected);
    }

}
