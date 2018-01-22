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
    fn floor(v: f32) -> u32;
    fn console(v: usize);
}

#[no_mangle]
pub fn run(width: u32, height: u32) {

    let mut group = Box::new(Group::new());

    let event_loop: EventLoop = EventLoop::new(Box::new(move |event_loop| {
        let group = group.as_mut();

        unsafe {
            for _ in 0..floor(random() * 10.0) {
                let x = floor(random() * (width as f32));
                let y = 0.0;
                let speed= random() + 0.1;

                group.add(x, y, 3, speed);
            }
        }

        group.set_timestamp(event_loop.get_timestamp());

        for point in group.iter_mut() {
            
            point.clear();
            point.move_y();
            point.draw();

            if point.get_y() as u32 >= (height - 2) {
                point.remove_next_iter(event_loop.get_timestamp());
            }
        }

        group.evaluate_points_remove();

        event_loop.request_animation_frame();
    }));


    event_loop.request_animation_frame();
}