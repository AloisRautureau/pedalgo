//! Implementation of the Simplex algorithm
//! definition of the simplex object

pub mod constraint;
pub mod linear_function;
pub mod point;

use constraint::Constraints;
use linear_function::LinearFunction;

type LinearEquations = (LinearFunction, Constraints);

/// Simplex object
#[derive(Debug)]
pub struct Simplex {
    index: usize,
    state: LinearEquations,
    historic: Vec<LinearEquations>,
}

impl Simplex {
    fn is_first_step(&mut self) -> bool {
        self.index == 0
    }

    fn is_optimal(&mut self) -> bool {
        self.state.0.only_negative_coefficients()
    }

    fn pivot(&mut self) {
        // choix variable entrante
        // choix variable sortante
        // pivot
        // mise à jour des coefficients
        // mise à jour des contraintes
        // mise à jour de la fonction objectif
        todo!();
    }

    /// Next step of the Simplex algorithm
    pub fn next_step(&mut self){
        if !self.is_optimal() {
            self.index += 1;
            self.historic.push(self.state);
            self.pivot();
        }
        println!("{}", self.state);
    }

    pub fn last_step(&mut self) {
        if !self.is_first_step() {
            self.index -= 1;
            self.state = self.historic.pop().unwrap();
        }
        println!("{}", self.state);
    }
}

impl std::fmt::Display for LinearEquations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)?;
        writeln!(f, "{}", self.1);
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

    #[test]
    fn test_pivot() {
        todo!();
    }

    #[test]
    fn test_new() {
        todo!();
    }
}
