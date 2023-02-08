//! contraintes linéaire
use std::ops::Index;

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

	/// Normalizes a constraint with respect to a variable
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
			if (constraint.right.constant < min_constant) && (index < 0.0){
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

		let &right_coeff = new_constraints.inner[i]
										.right
										.index(var.clone());
		let (evar, left_coeff) = new_constraints.inner[i]
										.left
										.first_positive_coefficient();

		let right_compensation = LinearFunction::single_variable_with_coeff(var, right_coeff);
		let left_compensation = LinearFunction::single_variable_with_coeff(evar, left_coeff);
		new_constraints.inner[i] -= right_compensation;
		new_constraints.inner[i] -= left_compensation;

		let rhs = new_constraints.inner[i].right.clone();
		(new_constraints, rhs)
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
    /// assert_eq!(c+l_f, expected)
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
    ///
    /// let c = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let l_f = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(25f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), 12f32), (String::from("z"), 0f32)]));
	/// c += l_f;
    /// assert_eq!(c, expected)
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
    /// let expected = LinearFunction::new(35f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), 12f32), (String::from("z"), -10f32)]));
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
    ///
    /// let c = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let l_f = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(35f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), 12f32), (String::from("z"), -10f32)]));
	/// c -= l_f;
    /// assert_eq!(c, expected)
    /// ```
    fn sub_assign(&mut self, rhs: LinearFunction) {
		self.left -= rhs.clone();
		self.right -= rhs;
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
