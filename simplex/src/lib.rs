//! Implementation of the Simplex algorithm
//! definition of the simplex object

pub mod constraint;
pub mod linear_function;
pub mod point;

use constraint::Constraints;
use linear_function::LinearFunction;

#[derive(Debug, Clone)]
pub struct LinearEquations {
    linear_function: LinearFunction,
    constraints: Constraints,
}

/// Simplex object
#[derive(Debug)]
pub struct Simplex {
    index: usize,
    historic: Vec<LinearEquations>,
}

impl LinearEquations {
    pub fn pivot(&mut self, use_bland_rule: bool) -> LinearEquations {
        let current_state = self;

        if use_bland_rule {
        } else {
            todo!()
        }
        // linear_function.max_coeff() -> name of the variable

        // contraint - x
        // choix variable sortante
        // pivot
        // mise à jour des coefficients
        // mise à jour des contraintes
        // mise à jour de la fonction objectif
        todo!();
    }
}

impl Simplex {
    fn is_first_step(&mut self) -> bool {
        self.index == 0
    }

    fn is_optimal(&mut self) -> bool {
        self.historic[self.index]
            .linear_function
            .only_negative_coefficients()
    }

    pub fn next_step(&mut self, use_bland_rule: bool) {
        if !self.is_optimal() && (self.index == self.historic.len() - 1) {
            self.index += 1;
            let new_state = self.historic[self.index].pivot(use_bland_rule);
            self.historic.push(new_state);
        }
        println!("{}", self.historic[self.index]);
    }

    pub fn last_step(&mut self) {
        if !self.is_first_step() {
            self.index -= 1;
        }
        println!("{}", self.historic[self.index]);
    }
}

impl std::fmt::Display for LinearEquations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.linear_function)?;
        writeln!(f, "{}", self.constraints)
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
