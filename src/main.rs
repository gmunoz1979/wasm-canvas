mod group;
mod point;
mod eventloop;

use group::Group;
use point::Point2D;
use eventloop::EventLoop;
pub use eventloop::event_loop;

fn main() { }

extern "C" {
    fn random() -> f32;
    fn floor(v: f32) -> u16;
    //fn console(v: usize);
}

#[no_mangle]
pub fn run(width: u16, height: u16) {
    let mut key: u32 = 0;
    let size: u8 = 3;
    let mut group = Box::new(Group::new());

    let event_loop: EventLoop = EventLoop::new(Box::new(move |event_loop| {
        let group = group.as_mut();

        unsafe {
            for _ in 0..floor(random() * 5.0) {
                let x: u16 = floor(random() * (width as f32));
                let y: f32 = 0.0;
                let speed: f32 = random() + 0.1;

                let mut point = Point2D::new(x, y, size, speed);
                point.set_index(key.clone());
                
                group.add(key.clone(), point);

                key += 1;
            }
        }

        group.set_timestamp(event_loop.get_timestamp());

        group.iter_mut().for_each(|point: &mut Point2D| {
            point.clear();
            point.move_y();
            point.draw();

            if point.get_y() as u16 >= (height - 2) {
                point.remove_next_iter(event_loop.get_timestamp());
            }
        });

        group.evaluate_points_remove();

        event_loop.request_animation_frame();
    }));


    event_loop.request_animation_frame();
}