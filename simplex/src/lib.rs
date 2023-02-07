//! Implementation of the Simplex algorithm
//! definition of the simplex object

mod constraint;
mod linear_function;
mod point;

/// Simplex object
#[derive(Debug)]
pub struct Simplex {
    l_function: linear_function::Linear_function,
    constraints: constraint::Constraints,
    index: usize,
    state: (linear_funtion::Linear_function, constrait::Constraints),
    historic: Vec<(linear_function::Linear_function, constraint::Constraint)>,
}

impl Simplex {
    fn is_first_step(&mut self) -> bool {
        self.index == 0
    }

    fn is_optimal(&mut self) -> bool {
        linear_function::all_is_negative(self.state.0)
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
    fn test_is_first_step(&self) -> bool {
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
