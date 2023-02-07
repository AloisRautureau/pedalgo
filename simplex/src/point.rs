//! points en n dimensions

/// Points in n dimensions
#[derive(Debug)]
pub struct Point {
    pub coordinates: Vec<f32>,
}

// surcharge de l'opérateur +

/// Overload of the + operator

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, p: Point) -> Self::Output {
        let mut coordinates = Vec::new();
        for i in 0..self.coordinates.len() {
            coordinates.push(self.coordinates[i] + p.coordinates[i]);
        }
        Point { coordinates }
    }
}
