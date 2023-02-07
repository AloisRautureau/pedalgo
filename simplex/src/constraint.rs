//! contraintes linéaire

/// Contraintes object

#[derive(Debug)]
enum Operator {
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

/// A Constraint is a linear function with an operator
/// [linear_function] [operator] [0]
#[derive(Debug)]
struct Constraint {
    pub left: linear_function::LinearFunction,
    pub operator : Operator,
}

pub type Constraints = Vec<Constraint>;

pub impl Constraint {
    pub fn new(left: linear_function::LinearFunction, operator: Operator, right: linear_function::LinearFunction) -> Constraint {
        Constraint {left: left - right , operator}
    }
}

/*
------------------            _____
max x + 3y;      |           | RUN |
2x - 5y <= 10;   |            -----
x + y <= 5;      |
x <= 0;          |
-----------------

ET( linear_function, OP(l_f, operor, l_f),  
*/
