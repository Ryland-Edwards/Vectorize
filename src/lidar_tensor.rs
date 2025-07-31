pub struct LidarTensor {
     pub point_data: Vec<Point>, //<<(x1,y1)>,<(x2,y2)>,...>

}

pub struct VectorData {
    pub point1_data: Point,
    pub point2_data: Point,
    pub magnitude: f64,
    pub dist_to_origin: f64,

}
#[derive(Debug)]
pub enum Point {
    PointData(f64, f64),
}

impl LidarTensor {
    pub fn new() -> Self {
        LidarTensor {
            point_data: Vec::new(),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.point_data.push(point);
    }

    pub fn print_points(&self) {
        for point in &self.point_data {
            println!(" {:?}", point);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_data_test() {
        let mut lidar = LidarTensor::new();

        // Create a Point enum instance
        let point = Point::PointData(1.0, 2.0);

        // Add it to the struct
        lidar.add_point(point);

        // Pattern matching to work with the enum
        lidar.print_points();
    }
}