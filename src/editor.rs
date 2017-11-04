use std;
use std::path::Path;
use std::collections::vec_deque::{VecDeque, Drain};

use vst2::editor::{Editor, KeyCode, KnobMode};
use libc::c_void;
use libc;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 640;

pub struct DelayEditor {
    current_delay: f32,
    changed: bool,
}

impl DelayEditor {
    pub fn new() -> DelayEditor {
        DelayEditor {
            current_delay: 0.,
            changed: false
        }
    }

    pub fn get_current_delay(&mut self) -> Option<f32> {
        if self.changed {
            self.changed = false;
            Some(self.current_delay)
        } else {
            None
        }
    }

    pub fn set_delay(&mut self, value: f32) {
        self.current_delay = value;
    }
}


impl Editor for DelayEditor {
    fn size(&self) -> (i32, i32) {
        (WIDTH, HEIGHT)
    }
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }
    fn open(&mut self, window: *mut c_void) {}
    fn is_open(&mut self) -> bool {
        false
    }

    fn idle(&mut self) {}
    fn close(&mut self) {}
    fn set_knob_mode(&mut self, mode: KnobMode) -> bool {
        true
    }
    fn key_up(&mut self, keycode: KeyCode) -> bool {
        true
    }
    fn key_down(&mut self, keycode: KeyCode) -> bool {
        true
    }
}

