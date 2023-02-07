//! Implementation of the Simplex algorithm
//! definition of the simplex object


/// Simplex object
#[derive(Debug)]
pub struct simplex {
    l_function: linear_function,
    constraints: constraints,
    state: (linear_function, constraints),
    historic: Vec<(linear_function, constraints)>,
}


impl simplex() {
    
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

    fn is_optimal(&self) -> bool {
        todo!();
    }

    fn is_first_step(&self) -> bool {
        todo!();
    }
}


/// Implentation of the simplex algorithm
/// Input :
///    - A : matrix of the constraints
///    - b : vector of the constraints
///    - c : vector of the objective function  
