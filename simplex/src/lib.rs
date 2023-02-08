//! Implementation of the Simplex algorithm
//! definition of the simplex object

pub mod app;
pub mod constraint;
pub mod linear_function;

use constraint::Constraints;
use linear_function::LinearFunction;

#[derive(Debug, Clone)]
pub struct LinearProgram {
    pub linear_function: LinearFunction,
    pub constraints: Constraints,
}

/// Simplex object
#[derive(Debug)]
pub struct Simplex {
    index: usize,
    historic: Vec<LinearProgram>,
}

impl LinearProgram {
    pub fn pivot(&self, use_bland_rule: bool) -> LinearProgram {
        if use_bland_rule {
            // applies bland rule
            let (var, coeff) = self.linear_function.first_positive_coefficient();
            //
            let max_index = self.constraints.constraint_max(var.clone());
            //
            let (new_constraints, new_value_var) = self.constraints.pivot_with(var, max_index);

            LinearProgram {
                linear_function: self.linear_function.clone() + new_value_var * coeff,
                constraints: new_constraints,
            }
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
    }
}

impl From<LinearProgram> for Simplex {
    fn from(value: LinearProgram) -> Self {
        Simplex {
            index: 0,
            historic: vec![value],
        }
    }
}
impl Simplex {
    fn is_first_step(&self) -> bool {
        self.index == 0
    }

    fn is_optimal(&self) -> bool {
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

    /// Returns a reference to the current state of the algorithm
    pub fn current_state(&self) -> &LinearProgram {
        &self.historic[self.index]
    }
}

impl std::fmt::Display for LinearProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.linear_function)?;
        writeln!(f, "{}", self.constraints)
    }
}

/*
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

 */
