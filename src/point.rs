extern "C" {
    fn clearRect(x: u32, y: u32, width: u8, height: u8);
    fn fillRect(x: u32, y: u32, width: u8, height: u8);
}

pub struct Point2D {
    x: u32,
    y: f32,
    size: u8,
    speed:f32
}

impl Point2D {
    pub fn new(x: u32, y: f32, size: u8, speed: f32) -> Point2D {
        Point2D { x: x, y: y, size: size, speed: speed }
    }

    pub fn draw(&self) {
        unsafe {
            fillRect(self.x, self.y as u32, self.size, self.size);
        }
    }

    pub fn clear(&self) {
      unsafe {
          clearRect(self.x, self.y as u32, self.size, self.size);
      }
    }

    pub fn _move(&mut self) {
        self.y += self.speed;
    }

    pub fn get_y(&self) -> u32 {
      self.y as u32
    }
}