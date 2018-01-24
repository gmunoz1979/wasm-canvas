mod group;
mod point;
mod eventloop;

use std::mem;
use std::os::raw::c_void;
use std::slice;
use group::Group;
use point::Point2D;
use eventloop::EventLoop;
pub use eventloop::event_loop;

fn main() { }

extern "C" {
    fn random() -> f32;
    fn floor(v: f32) -> u32;
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub fn run(pointer: *mut u8, width: u32, height: u32) {

    let byte_size: usize = (width * height * 4) as usize;

    let mut key: u32 = 0;
    let size: u8 = 3;
    let mut group = Box::new(Group::new());

    let event_loop: EventLoop = EventLoop::new(Box::new(move |event_loop| {
        let group = group.as_mut();
        let sl = unsafe{ slice::from_raw_parts_mut(pointer, byte_size) };

        unsafe {
            for _ in 0..floor(random() * 15.0) {
                let x: u32 = floor(random() * (width as f32));
                let y: f32 = 0.0;
                let speed: f32 = (floor(random() * 5.0) + 1) as f32;

                let mut point = Point2D::new(x, y, size, speed);
                point.set_index(key.clone());
                
                group.add(key.clone(), point);

                key += 1;
            }
        }

        group.set_timestamp(event_loop.get_timestamp());

        group.iter_mut().for_each(|point: &mut Point2D| {
            point.move_y();

            if point.get_y() as u16 >= (height - 2) as u16 {
                point.remove_next_iter(event_loop.get_timestamp());
            }
        });

        group.evaluate_points_remove();

        // Clear
        for i in 0..byte_size {
            sl[i] = 0;
        }

        group.iter().for_each(|point: &Point2D| {
            let i = (point.get_x() + (point.get_y() * width)) * 4;

            for x in 0..point.get_size() - 1 {
                for y in 0..point.get_size() - 1 {
                    let offset: u32 = (x as u32 + (y as u32 * width)) * 4;

                    if ((i + offset + 3) as usize) <= sl.len() {
                        sl[(i + offset + 0) as usize] = 0;
                        sl[(i + offset + 1) as usize] = 255;
                        sl[(i + offset + 2) as usize] = 0;
                        sl[(i + offset + 3) as usize] = (point.get_speed() * 255.0) as u8;
                    }

                }
            }
        });

        event_loop.request_animation_frame();
    }));


    event_loop.request_animation_frame();
}