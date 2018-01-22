use std::cell::RefCell;
use std::collections::HashMap;

extern "C" {
    fn requestAnimationFrame(id: u32);
}


#[no_mangle]
pub fn event_loop(id: u32, timestamp: u32) {
    EVENTLOOPS.with(|el| {
        let mut el = el.borrow_mut();
        if let Some(cb) = el.get_mut(&id) {
            cb(EventLoop { id: id, timestamp: timestamp });
        }
    });
}

thread_local!(
    static EVENTLOOPS: RefCell<HashMap<u32, Box<FnMut(EventLoop)>>> = RefCell::new(HashMap::new())
);

pub struct EventLoop {
    id: u32,
    timestamp: u32
}

impl EventLoop {
    pub fn new(cb: Box<FnMut(EventLoop)>) -> EventLoop {
        let id = 0;

        EVENTLOOPS.with(|el| {
            el.borrow_mut().insert(id, cb)
        });

        EventLoop { id: id, timestamp: 0 }
    }

    pub fn request_animation_frame(&self) {
        unsafe {
            requestAnimationFrame(self.id);
        }
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }
}