use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};

use nom::multi::many0;
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
        LinearFunction::default()
    }

    /// Creates a new linear function containing a single variable with coefficient 1
    pub fn single_variable(var: Variable) -> LinearFunction {
        LinearFunction {
            constant: 0f32,
            coefficients: HashMap::from([(var, 1f32)]),
        }
    }

    /// Creates a new linear function containing a single variable with a predefinite coefficient
    pub fn single_variable_with_coeff(var: Variable, coeff: f32) -> LinearFunction {
        LinearFunction {
            constant: 0f32,
            coefficients: HashMap::from([(var, coeff)]),
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
        // self.coefficients
        // .clone()
        // .into_iter()
        // .find(|(_, c)| !c.is_sign_negative())
        // .expect("searched for a positive coefficient on a constant linear function")

        let mut h_map: Vec<_> = self.coefficients.clone().into_iter().collect();
        h_map.sort_by_key(|(var, _)| var.clone());
        h_map.retain(|(_, coeff)| *coeff != 0.0);
        let coeff_iter = h_map.iter();

        for (var, coeff) in coeff_iter {
            if *coeff > 0.0 {
                return (var.to_string(), *coeff);
            }
        }

        ("error".to_string(), 0.0)
    }

    /// Normalizes a linear function with respect to a variable (be careful as it normalizes with a negative one before the variable)
    pub fn normalize(&self, var: Variable) -> (LinearFunction, Coefficient) {
        let mut func = self.clone();
        let var_coeff = if let Some(var_coeff) = self.coefficients.get(&var).copied() {
            var_coeff
        } else {
            return (func, 0.0);
        };
        //.expect("Unknown variable in linear function");

        for (variable, coeff) in self.coefficients.iter() {
            func[variable.to_string()] = -1f32 * coeff / var_coeff;
        }

        func[var] = -1f32;
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
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let mut c = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let l_f = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(25f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), 12f32), (String::from("z"), 0f32)]));
    /// c += l_f.clone();
    /// assert_eq!(c, expected)
    /// ```
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
    /// ```rust
    /// use std::collections::HashMap;
    /// use simplex::linear_function::LinearFunction;
    ///
    /// let mut c = LinearFunction::new(30f32, HashMap::from([(String::from("x"), 32f32), (String::from("z"), -5f32)]));
    /// let l_f = LinearFunction::new(-5f32, HashMap::from([(String::from("y"), 12f32), (String::from("z"), 5f32)]));
    /// let expected = LinearFunction::new(35f32, HashMap::from([(String::from("x"), 32f32), (String::from("y"), -12f32), (String::from("z"), -10f32)]));
    /// c -= l_f;
    /// assert_eq!(c, expected)
    /// ```
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

            let mut found_coeff = false;
            let (rest, coeff) =
                if let Ok((rest, coeff)) = preceded(multispace0::<&str, ()>, float)(rest) {
                    found_coeff = true;
                    (rest, coeff)
                } else {
                    (rest, 1.0)
                };

            let (rest, variable) = match preceded(multispace0::<&str, ()>, alpha1)(rest) {
                Ok((rest, variable)) => (rest, variable.to_string()),
                Err(_) if found_coeff => (rest, String::new()),
                _ => {
                    return Err(nom::Err::Error(nom::error::Error {
                        input: "aled",
                        code: nom::error::ErrorKind::Fail,
                    }))
                }
            };

            Ok((rest, (variable, if positive { coeff } else { -coeff })))
        }

        let mut linear_func = LinearFunction::zero();
        let (_, variables) = many0(parse_variable)(s).unwrap();
        for (var, coeff) in variables {
            if var.is_empty() {
                linear_func.constant += coeff;
            } else {
                linear_func[var] += coeff;
            }
        }
        Ok(linear_func)
    }
}

impl std::fmt::Display for LinearFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // sort the hashmap by variable name
        // filtre for the non-zero coefficients
        // then iterate over the coefficients
        let mut h_map: Vec<_> = self.coefficients.clone().into_iter().collect();
        h_map.sort_by_key(|(var, _)| var.clone());
        h_map.retain(|(_, coeff)| *coeff != 0.0);
        let mut coeff_iter = h_map.iter();

        if self.constant != 0.0 {
            write!(f, "{}", self.constant)
        } else if let Some((var, coeff)) = coeff_iter.next() {
            match *coeff {
                x if x == 1.0 => write!(f, "{var}"),
                x if x == -1.0 => write!(f, "-{var}"),
                _ => write!(f, "{coeff}{var}"),
            }
        } else {
            write!(f, "0")
        }?;
        for (var, coeff) in coeff_iter {
            match *coeff {
                x if x == 1.0 => write!(f, " + {var}"),
                x if x == -1.0 => write!(f, " - {var}"),
                _ => write!(
                    f,
                    "{}{}{var}",
                    if coeff.is_sign_positive() {
                        " + "
                    } else {
                        " - "
                    },
                    coeff.abs(),
                ),
            }?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_single_variable_with_coeff() {
        let single_variable_lf = LinearFunction::single_variable_with_coeff("x".to_string(), 32f32);
        let expected = LinearFunction::from_str("32x").unwrap();

        assert_eq!(single_variable_lf, expected);
    }

    #[test]
    fn test_first_positive_coefficient() {
        let lf = LinearFunction::from_str("200+5x-6z+3y").unwrap();
        let var = "x".to_string();
        let coeff = 5.0;

        assert_eq!(lf.first_positive_coefficient(), (var, coeff));
    }

    #[test]
    fn test_normalize() {
        let lf = LinearFunction::from_str("3x+6y-9z+150").unwrap();
        let var = "x".to_string();
        let expected = LinearFunction::from_str("-x-2y+3z-50").unwrap();

        let (normalized_lf, var_coeff) = lf.normalize(var);

        assert_eq!((normalized_lf, var_coeff), (expected, 3.0));
    }
}
