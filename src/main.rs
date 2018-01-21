mod point;
mod eventloop;

use std::collections::HashMap;
use point::Point2D;
use eventloop::EventLoop;
pub use eventloop::event_loop;

fn main() { }

extern "C" {
    fn random() -> f32;
    fn floor(v: f32) -> u32;
    fn console(v: usize);
}

#[no_mangle]
pub fn run(width: u32, height: u32) {

    let mut group: Box<HashMap<u32, Point2D>> = Box::new(HashMap::new());
    let mut index: u32 = 0;

    let event_loop: EventLoop = EventLoop::new(Box::new(move |event_loop| {
        let group = group.as_mut();

        unsafe {
            for _ in 0..floor(random() * 10.0) {
                let x = floor(random() * (width as f32));
                let mut point = Point2D::new(x, 0 as f32, 4, random() + 0.1);
                group.insert(index, point);
                index +=1;
            }
        }

        let mut remove: Vec<&u32> = Vec::new();

        {
            for (k, v) in group.iter_mut() {
            v.clear();
            v._move();
            v.draw();

            if v.get_y() as u32 > height {
                remove.push(&k);
            }
            }
        }

        // for i in &remove {
        //     group.remove(&i);
        // }
 
        // unsafe {
        //     console(group.len());
        // }

        event_loop.request_animation_frame();
    }));

    event_loop.request_animation_frame();
}