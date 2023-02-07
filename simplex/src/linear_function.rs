use std::collections::HashMap;

pub type Variable = String;

pub struct LinearFunction {
    coefficients: HashMap<Variable, f32>,
    constant: f32
}
impl LinearFunction {
    /// Applies the linear function to a given valuation, returning the value
    pub fn apply(&self, valuation: &HashMap<Variable, f32>) -> f32 {
        self.coefficients
            .iter()
            .fold(self.constant, |acc, (var, coeff)| {
                acc + (valuation.get(var).unwrap_or(&0f32) * coeff)
            })
    }
}


impl std::str::FromStr for LinearFunction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl std::fmt::Display for LinearFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/*
OPERATOR OVERLOADING
 */
impl std::ops::Add<LinearFunction> for LinearFunction {
    type Output = LinearFunction;

    fn add(self, rhs: LinearFunction) -> Self::Output {
        todo!()
    }
}
impl std::ops::AddAssign<LinearFunction> for LinearFunction {
    fn add_assign(&mut self, rhs: LinearFunction) {
        todo!()
    }
}

impl std::ops::Sub<LinearFunction> for LinearFunction {
    type Output = LinearFunction;

    fn sub(self, rhs: LinearFunction) -> Self::Output {
        todo!()
    }
}
impl std::ops::SubAssign<LinearFunction> for LinearFunction {
    fn sub_assign(&mut self, rhs: LinearFunction) {
        todo!()
    }
}

impl std::ops::Mul<LinearFunction> for LinearFunction {
    type Output = LinearFunction;

    fn mul(self, rhs: LinearFunction) -> Self::Output {
        todo!()
    }
}
impl std::ops::MulAssign<LinearFunction> for LinearFunction {
    fn mul_assign(&mut self, rhs: LinearFunction) {
        todo!()
    }
}
impl std::ops::Mul<f32> for LinearFunction {
    type Output = LinearFunction;

    fn mul(self, rhs: f32) -> Self::Output {
        todo!()
    }
}
impl std::ops::MulAssign<f32> for LinearFunction {
    fn mul_assign(&mut self, rhs: f32) {
        todo!()
    }
}

impl std::ops::Div<LinearFunction> for LinearFunction {
    type Output = LinearFunction;

    fn div(self, rhs: LinearFunction) -> Self::Output {
        todo!()
    }
}
impl std::ops::DivAssign<LinearFunction> for LinearFunction {
    fn div_assign(&mut self, rhs: LinearFunction) {
        todo!()
    }
}
impl std::ops::Div<f32> for LinearFunction {
    type Output = LinearFunction;

    fn div(self, rhs: f32) -> Self::Output {
        todo!()
    }
}
impl std::ops::DivAssign<f32> for LinearFunction {
    fn div_assign(&mut self, rhs: f32) {
        todo!()
    }
}

impl std::ops::Neg for LinearFunction {
    type Output = LinearFunction;

    fn neg(self) -> Self::Output {
        todo!()
    }
}