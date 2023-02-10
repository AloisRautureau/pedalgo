#[derive(Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    /// Always project on the (x, y) plane, as we expect the camera to always look along -z
    pub fn project_on_xy(&self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();

        vec.push(self.x);
        vec.push(self.y);
        vec.push(0.0);

        vec
    }

    pub fn rotate_around_x(&mut self, angle: f32) {
        let cos = angle.cos();
        let sin = angle.sin();

        self.y = cos * self.y - sin * self.z;
        self.z = sin * self.y + cos * self.z;
    }

    pub fn rotate_around_y(&mut self, angle: f32) {
        let cos = angle.cos();
        let sin = angle.sin();

        self.x = cos * self.x + sin * self.z;
        self.z = cos * self.z - sin * self.x;
    }

    pub fn rotate_around_z(&mut self, angle: f32) {
        let cos = angle.cos();
        let sin = angle.sin();

        self.x = cos * self.x - sin * self.y;
        self.y = sin * self.x + cos * self.y;
    }
}

pub struct SimplexPoints {
    points: Vec<Point>,
}

impl SimplexPoints {
    pub fn new(points: Vec<Point>) -> SimplexPoints {
        SimplexPoints { points }
    }

    pub fn project_on_xy(&self) -> Vec<Vec<f32>> {
        self.points
            .clone()
            .into_iter()
            .map(|point| point.project_on_xy())
            .collect()
    }

    pub fn rotate_around_x(&mut self, angle: f32) {
        self.points
            .iter_mut()
            .for_each(|point| point.rotate_around_x(angle));
    }

    pub fn rotate_around_y(&mut self, angle: f32) {
        self.points
            .iter_mut()
            .for_each(|point| point.rotate_around_y(angle));
    }

    pub fn rotate_around_z(&mut self, angle: f32) {
        self.points
            .iter_mut()
            .for_each(|point| point.rotate_around_z(angle))
    }
}
