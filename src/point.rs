extern "C" {
    fn clearRect(x: u16, y: u16, width: u8, height: u8);
    fn fillRect(x: u16, y: u16, width: u8, height: u8);
    fn setFillStyle(speed: f32, remove: bool);
}

#[derive(Debug)]
pub struct Point2D {
    x:         u16,
    y:         f32,
    size:      u8,
    speed:     f32,
    remove:    bool,
    timestamp: u32,
    index:     u32
}

impl Point2D {
    pub fn new(x: u16, y: f32, size: u8, speed: f32) -> Point2D {
        Point2D { 
            x:         x, 
            y:         y, 
            size:      size, 
            speed:     speed, 
            remove:    false,
            timestamp: 0,
            index:     0
        }
    }

    pub fn draw(&self) {
        unsafe {
            setFillStyle(self.speed, self.remove);
            fillRect(self.x , self.y as u16, self.size, self.size);
        }
    }

    pub fn clear(&self) {
      unsafe {
          clearRect(self.x, self.y as u16, self.size, self.size);
      }
    }

    pub fn move_y(&mut self) {
        if !self.is_remove() {
            self.y += self.speed;
        }
    }

    pub fn get_y(&self) -> u32 {
      self.y as u32
    }

    pub fn remove_next_iter(&mut self, timestamp: u32) {
        if !self.remove {
            self.remove = true;
            self.timestamp = timestamp;
        }
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn is_remove(&self)-> bool {
        self.remove
    }

    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    pub fn get_index(&self) -> &u32 {
        &self.index
    }
}