//! points en n dimensions

/// Points in n dimensions
#[derive(Debug)]
pub struct Point {
    l_function: linear_function,
    constraints: constraints,
    state: (linear_function, constraints),
    historic: Vec<(linear_function, constraints)>,
}