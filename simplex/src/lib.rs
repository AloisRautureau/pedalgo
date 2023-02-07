//! Implementation of the Simplex algorithm
//! definition of the simplex object

pub mod constraint;
pub mod linear_function;
pub mod point;

use constraint::Constraints;
use linear_function::LinearFunction;

/// Simplex object
#[derive(Debug)]
pub struct Simplex {
    l_function: LinearFunction,
    constraints: Constraints,
    index: usize,
    state: (LinearFunction, Constraints),
    historic: Vec<(LinearFunction, Constraints)>,
}

impl Simplex {
    fn is_first_step(&mut self) -> bool {
        self.index == 0
    }

    fn is_optimal(&mut self) -> bool {
        self.state.0.only_negative_coefficients()
    }

    fn pivot(&mut self) {
        todo!();
    }

    /// Next step of the Simplex algorithm
    pub fn next_step(&mut self) {
        todo!();
        // function : pivot
        // function : clean_print (Terminal then GUI)
    }

    pub fn last_step(&mut self) {
        todo!();
        // function : pivot
        // function : clean_print (Terminal then GUI)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_first_step() {
        todo!();
    }

    #[test]
    fn test_is_optimal() {
        todo!();
    }

    #[test]
    fn test_next_step() {
        todo!();
    }

    #[test]
    fn test_last_step() {
        todo!();
    }
}
