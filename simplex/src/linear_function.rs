use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};

use nom::number::complete::float;
use nom::sequence::preceded;
use nom::IResult;

pub type Variable = String;
pub type Coefficient = f32;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct LinearFunction {
    pub constant: Coefficient,
    coefficients: HashMap<Variable, Coefficient>,
}
impl LinearFunction {
    /// Creates a new linear function with the given constant and coefficients
    pub fn new(constant: f32, coefficients: HashMap<Variable, Coefficient>) -> LinearFunction {
        LinearFunction {
            constant,
            coefficients,
        }
    }

    pub fn zero() -> LinearFunction {
        LinearFunction {
            constant: 0f32,
            coefficients: HashMap::default(),
        }
    }

    /// Creates a new linear function containing a single variable with coefficient 1
    pub fn single_variable(var: Variable) -> LinearFunction {
        LinearFunction {
            constant: 0f32,
            coefficients: HashMap::from([(var, 1f32)]),
        }
    }

    /// Applies the linear function to a given valuation, returning the value
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    /// let linear_func = LinearFunction::new(10f32, HashMap::from([("x".to_string(), 20f32), ("z".to_string(), -2f32)]));
    /// let valuation = HashMap::from([
    ///     (String::from("x"), 2f32),
    ///     (String::from("y"), -432f32)
    /// ]);
    /// assert_eq!(linear_func.apply(&valuation), 50f32)
    /// ```
    pub fn apply(&self, valuation: &HashMap<Variable, Coefficient>) -> f32 {
        self.coefficients
            .iter()
            .fold(self.constant, |acc, (var, coeff)| {
                acc + (valuation.get(var).unwrap_or(&0f32) * coeff)
            })
    }

    /// Returns true if the function only has negative coefficients
    pub fn only_negative_coefficients(&self) -> bool {
        for coeff in self.coefficients.values() {
            if !coeff.is_sign_negative() {
                return false;
            }
        }
        true
    }

    /// Returns the variable with the maximal coefficient
    pub fn max_coefficient(&self) -> (Variable, Coefficient) {
        self.coefficients
            .clone()
            .into_iter()
            .max_by(|(_, coeff_x), (_, coeff_y)| coeff_x.total_cmp(coeff_y))
            .expect("searched for a max coefficient on a constant linear function")
    }

    /// Returns the first variable with a positive coefficient
    pub fn first_positive_coefficient(&self) -> (Variable, Coefficient) {
        self.coefficients
            .clone()
            .into_iter()
            .find(|(_, c)| !c.is_sign_negative())
            .expect("searched for a positive coefficient on a constant linear function")
    }

	/// Normalizes a linear function with respect to a variable
	pub fn normalize(&self, var: Variable) -> (LinearFunction, Coefficient) {
		let mut func = self.clone();
		let var_coeff = self.coefficients
							.get(&var)
							.copied()
							.expect("Unknown variable in linear function");
		
		for (variable, coeff) in self.coefficients.iter() {
			func[variable.to_string()] = -1f32 * coeff / var_coeff;
		}

		func[var] = 1f32;
		func.constant /= var_coeff;
		func.constant *= -1f32;

		(func, var_coeff)
	}
}

impl std::ops::Index<Variable> for LinearFunction {
    type Output = Coefficient;

    fn index(&self, index: Variable) -> &Self::Output {
        self.coefficients.get(&index).unwrap_or(&0f32)
    }
}
impl std::ops::IndexMut<Variable> for LinearFunction {
    fn index_mut(&mut self, index: Variable) -> &mut Self::Output {
        self.coefficients.entry(index).or_insert(0f32)
    }
}

/*
OPERATOR OVERLOADING
 */
impl std::ops::Add<LinearFunction> for LinearFunction {
    type Output = LinearFunction;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let a = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let b = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(25f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), 12f32), (String::from("z"), 0f32)]));
    /// assert_eq!(a + b, expected)
    /// ```
    fn add(self, rhs: LinearFunction) -> Self::Output {
        let mut coefficients = self.coefficients;
        for (var, coeff) in rhs.coefficients {
            *coefficients.entry(var).or_insert(0f32) += coeff
        }

        LinearFunction {
            constant: self.constant + rhs.constant,
            coefficients,
        }
    }
}
impl std::ops::AddAssign<LinearFunction> for LinearFunction {
    fn add_assign(&mut self, rhs: LinearFunction) {
        self.constant += rhs.constant;
        for (var, coeff) in rhs.coefficients {
            *self.coefficients.entry(var).or_insert(0f32) += coeff
        }
    }
}

impl std::ops::Sub<LinearFunction> for LinearFunction {
    type Output = LinearFunction;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let a = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let b = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(35f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), -12f32), (String::from("z"), -10f32)]));
    /// assert_eq!(a - b, expected)
    /// ```
    fn sub(self, rhs: LinearFunction) -> Self::Output {
        let mut coefficients = self.coefficients;
        for (var, coeff) in rhs.coefficients {
            *coefficients.entry(var).or_insert(0f32) -= coeff
        }

        LinearFunction {
            constant: self.constant - rhs.constant,
            coefficients,
        }
    }
}
impl std::ops::SubAssign<LinearFunction> for LinearFunction {
    fn sub_assign(&mut self, rhs: LinearFunction) {
        self.constant -= rhs.constant;
        for (var, coeff) in rhs.coefficients {
            *self.coefficients.entry(var).or_insert(0f32) -= coeff
        }
    }
}

impl std::ops::Mul<f32> for LinearFunction {
    type Output = LinearFunction;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let a = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let expected = LinearFunction::new(60f32, HashMap::from([(String::from("x"), 64f32), (String::from("z"), -10f32)]));
    /// assert_eq!(a * 2f32, expected)
    /// ```
    fn mul(self, rhs: f32) -> Self::Output {
        LinearFunction {
            constant: self.constant * rhs,
            coefficients: self
                .coefficients
                .iter()
                .map(|(var, coeff)| (var.to_string(), coeff * rhs))
                .collect(),
        }
    }
}
impl std::ops::MulAssign<f32> for LinearFunction {
    fn mul_assign(&mut self, rhs: f32) {
        self.coefficients
            .values_mut()
            .for_each(|coeff| *coeff *= rhs);
        self.constant *= rhs
    }
}

impl std::ops::Div<f32> for LinearFunction {
    type Output = LinearFunction;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let a = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let expected = LinearFunction::new(15f32, HashMap::from([(String::from("x"), 16f32), (String::from("z"), -2.5)]));
    /// assert_eq!(a / 2f32, expected)
    /// ```
    fn div(self, rhs: f32) -> Self::Output {
        LinearFunction {
            constant: self.constant / rhs,
            coefficients: self
                .coefficients
                .iter()
                .map(|(var, coeff)| (var.to_string(), coeff / rhs))
                .collect(),
        }
    }
}
impl std::ops::DivAssign<f32> for LinearFunction {
    fn div_assign(&mut self, rhs: f32) {
        self.coefficients
            .values_mut()
            .for_each(|coeff| *coeff /= rhs);
        self.constant /= rhs
    }
}

impl std::ops::Neg for LinearFunction {
    type Output = LinearFunction;

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let a = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let expected = LinearFunction::new(-30f32, HashMap::from([(String::from("x"), -32f32), (String::from("z"), 5f32)]));
    /// assert_eq!(-a, expected)
    /// ```
    fn neg(self) -> Self::Output {
        LinearFunction {
            constant: -self.constant,
            coefficients: self
                .coefficients
                .iter()
                .map(|(var, coeff)| (var.to_string(), -coeff))
                .collect(),
        }
    }
}

/*
PARSE FUNCTIONS
 */
impl std::str::FromStr for LinearFunction {
    type Err = ();

    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let expected = LinearFunction::new(30f32, HashMap::from([(String::from("x"), -32f32), (String::from("z"), 2.5f32)]));
    /// assert_eq!("2.5z + 30 - 32x".parse::<LinearFunction>().unwrap(), expected)
    /// ```
    /// TODO: Clean this
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_variable(input: &str) -> IResult<&str, (Variable, Coefficient)> {
            let (rest, positive) = if let Ok((rest, sign)) =
                preceded(multispace0::<&str, ()>, alt((tag("-"), tag("+"))))(input)
            {
                (rest, sign == "+")
            } else {
                (input, true)
            };

            let (rest, coeff) =
                if let Ok((rest, coeff)) = preceded(multispace0::<&str, ()>, float)(rest) {
                    (rest, coeff)
                } else {
                    (rest, 1f32)
                };

            let (rest, variable) =
                if let Ok((rest, variable)) = preceded(multispace0::<&str, ()>, alpha1)(rest) {
                    (rest, variable.to_string())
                } else {
                    (rest, String::new())
                };

            Ok((rest, (variable, if positive { coeff } else { -coeff })))
        }

        let mut s = s;
        let mut linear_func = LinearFunction::zero();
        while let Ok((rest, (var, coeff))) = parse_variable(s) {
            if var.is_empty() {
                linear_func.constant = coeff;
            } else {
                linear_func[var] = coeff;
            }
            if rest.is_empty() {
                break;
            }
            s = rest;
        }
        Ok(linear_func)
    }
}

impl std::fmt::Display for LinearFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn coeff_to_string(var: &str, coeff: &Coefficient) -> String {
            format!(
                "{}{var}",
                if coeff == &1f32 {
                    String::from("+ ")
                } else if coeff.is_sign_negative() {
                    format!("- {}", coeff.abs())
                } else {
                    format!("+ {coeff}")
                }
            )
        }
        write!(
            f,
            "{} {}",
            self.constant,
            self.coefficients
                .iter()
                .fold(String::new(), |acc, (var, coeff)| {
                    acc + &coeff_to_string(var, coeff) + " "
                })
        )
    }
}
