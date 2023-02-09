//! Implementation of the Simplex algorithm
//! definition of the simplex object

pub mod app;
pub mod constraint;
pub mod linear_function;
mod polyhedron;

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
    pub fn pivot(&mut self, use_bland_rule: bool) {

        if let Some(var) = self.linear_function.first_positive_coefficient(use_bland_rule) {
            let max_constraint_index = self.constraints.most_restrictive(&var).expect(&format!("variable {var} does not appear in any constraint, and is therefore unbounded"));

            self.constraints.pivot(max_constraint_index, &var);
            self.linear_function.replace(&var, &self.constraints[max_constraint_index].right)
        }
    }

    pub fn is_optimal(&self) -> bool {
        self.linear_function.no_positive_coefficient()
    }
}
impl From<LinearProgram> for Simplex {
    fn from(program: LinearProgram) -> Self {
        Simplex {
            index: 0,
            historic: vec![program],
        }
    }
}
impl Simplex {
    fn is_first_step(&self) -> bool {
        self.index == 0
    }

    pub fn next_step(&mut self, use_bland_rule: bool) {
        if !self.current_state().is_optimal() {
            if self.index == self.historic.len() - 1 {
                let mut new = self.current_state().clone();
                new.pivot(use_bland_rule);
                self.historic.push(new);
            }
            self.index += 1
        }
    }

    pub fn previous_step(&mut self) {
        if !self.is_first_step() {
            self.index -= 1;
        }
    }

    /// Returns a reference to the current state of the algorithm
    pub fn current_state(&self) -> &LinearProgram {
        &self.historic[self.index]
    }
}

impl std::fmt::Display for LinearProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "max {}", self.linear_function)?;
        writeln!(f, "{}", self.constraints)
    }
}