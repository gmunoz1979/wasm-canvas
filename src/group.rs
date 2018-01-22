use std::collections::HashMap;
use std::collections::hash_map::ValuesMut;
use point::Point2D;

#[derive(Debug)]
pub struct Group {
    points:    HashMap<i32, Point2D>,
    timestamp: u32,
    index:     i32
}

impl Group {
    pub fn new() -> Group {
        Group { points: HashMap::new(), timestamp: 0, index: -1 }
    }

    pub fn add(&mut self, x: u32, y: f32, size: u8, speed: f32) {
        let mut point = Point2D::new(x, y, size, speed);

        self.index += 1;

        point.set_index(self.index);

        self.points.insert(self.index, point);
    }

    pub fn iter_mut(&mut self) -> ValuesMut<i32, Point2D> {
        self.points.values_mut()
    }

    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    fn get_index(&self) -> Vec<i32> {
        let mut index:Vec<i32> = Vec::new();

        for (k, v) in &self.points {
            if v.is_remove() && (self.timestamp - v.get_timestamp() > 1000) {
                index.push(v.get_index());
            }
        }

        index
    }

    pub fn evaluate_points_remove(&mut self) {
        let index = self.get_index();

        for v in index {
            self.points.remove(&v);
        }
    }
}

