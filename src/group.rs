use std::collections::HashMap;
use std::collections::hash_map::ValuesMut;
use point::Point2D;

#[derive(Debug)]
pub struct Group {
    points:    HashMap<u32, Point2D>,
    timestamp: u32
}

impl Group {
    pub fn new() -> Group {
        Group { points: HashMap::new(), timestamp: 0 }
    }

    pub fn add(&mut self, key: u32, point: Point2D) {
        self.points.insert(key, point);
    }

    pub fn iter_mut(&mut self) -> ValuesMut<u32, Point2D> {
        self.points.values_mut()
    }

    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    fn get_index(&self) -> Vec<u32> {
        let mut index:Vec<u32> = Vec::new();

        self.points.iter().for_each(|(_, v)| {
            if v.is_remove() && (self.timestamp - v.get_timestamp() > 1000) {
                index.push(*v.get_index());
            }
        });

        index
    }

    pub fn evaluate_points_remove(&mut self) {
        let index = self.get_index();

        for v in index {
            self.points.remove(&v);
        }
    }
}

