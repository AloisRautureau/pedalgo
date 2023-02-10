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
#[derive(Debug, Clone)]
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

    pub fn is_valid(&self) -> bool {
        self.constraints.is_valid()
    }

    /// only works on a proper linear program which is verif by is_valid function
    pub fn point(&self) -> Vec<f32> {
        if !self.is_valid() {
            panic!("Linear program is not valid");
        }
        let variables = self.non_gap_variables();
        let mut point = vec![0.0; variables.len()];

        for constraint in self.constraints.iter() {
            if let Some(index) = variables
                .iter()
                .position(|v| *v == constraint.left.name_single_variable())
            {
                point[index] = constraint.right.constant;
            }
        }
        point
    }

    /// Give every non gap variables of a linear program sorted by alphabetical order
    pub fn non_gap_variables(&self) -> Vec<String> {
        let mut variables = constraint::union(
            self.linear_function.non_gap_variables(),
            self.constraints.non_gap_variables(),
        );
        variables.sort();
        variables
    }
}

impl Simplex {
    fn is_first_step(&self) -> bool {
        self.index == 0
    }

    fn is_optimal(&self) -> bool {
        self.current_state()
            .linear_function
            .no_positive_coefficient()
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

    pub fn current_point(&self) -> Vec<f32> {
        self.current_state().point()
    }

    pub fn bfs_point(&self) -> Vec<Vec<f32>> {
        let mut points = Vec::new();
        points.push(self.current_point());
        let mut todo = Vec::<(LinearProgram, String)>::new();

        while !todo.is_empty() {
            let (programm, index) = todo.pop().unwrap();
            let point = programm.point();
            if !points.iter().any(|p| *p == point) {
                
            }
        }
        points
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

impl std::fmt::Display for LinearProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "max {}", self.linear_function)?;
        writeln!(f, "{}", self.constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_gap_variables() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x + y <= 2\n x + 2y <= 3").unwrap(),
        };
        assert_eq!(
            lp.non_gap_variables(),
            vec!["x".to_string(), "y".to_string()]
        );
    }

    #[test]
    fn test_point_1() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x + y <= 2\n x + 2y <= 3").unwrap(),
        };
        assert_eq!(lp.point(), vec![0.0, 0.0]);
    }

    #[test]
    // ne passe pas
    fn test_point_2() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x <= 200\n 300 - x + 2y >= 0").unwrap(),
        };
        let mut simplex = Simplex::from(lp);
        simplex.next_step(true);
        assert_eq!(simplex.current_point(), vec![200.0, 0.0]);
    }

    #[test]
    // ne passe pas
    fn test_bfs_point() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x <= 200\n 300 - x + 2y >= 0").unwrap(),
        };
        let mut simplex = Simplex::from(lp);
        assert_eq!(
            simplex.bfs_point(),
            vec![vec![0.0, 0.0], vec![200.0, 0.0], vec![200.0, 100.0]]
        );
    }
}
