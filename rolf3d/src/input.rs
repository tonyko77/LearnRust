//! InputManager - handles keyboard & mouse, knows if key/mousebtn is pressed, set key timings.

use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::mouse::*;
use std::collections::HashSet;

pub struct InputManager {
    // keep keys and buttons together, by converting their enum values to i32
    pressed: HashSet<i32>,

    // TODO: I probably need pixel size, to convert to actual pizels
    // TODO: I probably need some sort of "mouse capture", to see mouse movement in window mode
    //   => just use a key for mouse capture - e.g. F12 :)
    mouse_x: i32,
    mouse_y: i32,
    // mouse movement
    mouse_rel_x: i32,
    mouse_rel_y: i32,
    // TODO: key/btn repeat timings, or "press once"
    // use some sort of flags
    //timings: HashMap<i32, KeyBtnTiming>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            mouse_x: 0,
            mouse_y: 0,
            mouse_rel_x: 0,
            mouse_rel_y: 0,
        }
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        // TODO implement this:
        // store pressed/released keys/buttons
        // store mouse movement
        true
    }

    fn update_state(&mut self, elapsed_time: f64) {
        // TODO update states for repeating keys/btns
    }

    // TODO getters/checkers for input state
}

//------------------
//  Internal stuff

/// Structure for managing key/button repeats and single-shot behaviour.
/// Note: times are represented in milliseconds.
struct KeyBtnTiming {
    repeat_delay: u16,
    waiting_ms: u16,
    is_pressed: bool,
    wait_residual: f64,
}

impl KeyBtnTiming {
    fn new(repeat_delay: u16) -> Self {
        Self {
            repeat_delay: repeat_delay,
            waiting_ms: 0,
            is_pressed: false,
            wait_residual: 0.0,
        }
    }

    fn set_pressed(&mut self, pressed: bool) {
        if self.is_pressed != pressed {
            self.is_pressed = pressed;
            self.waiting_ms = 0;
            self.wait_residual = 0.0;
        }
    }

    fn update_elapsed(&mut self, elapsed: f64) {
        const MILLIS: f64 = 1e-3;
        // only update wait time if we didn't reach the threshold
        if self.is_pressed && self.repeat_delay > 0 {
            self.wait_residual += elapsed;
            let cnt_millis = (self.wait_residual / MILLIS) as u16;
            if cnt_millis > 0 {
                self.waiting_ms += cnt_millis;
                self.wait_residual -= (cnt_millis as f64) * MILLIS;
            }
        }
    }

    fn consume_pressed(&mut self) -> bool {
        if !self.is_pressed {
            return false;
        }
        if self.repeat_delay == 0 {
            // single shot behaviour
            if self.waiting_ms == 0 {
                self.waiting_ms = 1;
                return true;
            }
        } else {
            // repeat behaviour
            if self.waiting_ms >= self.repeat_delay {
                self.waiting_ms -= self.repeat_delay;
                return true;
            }
        }
        false
    }
}

#[inline(always)]
fn key2code(key: Keycode) -> i32 {
    key as i32
}

#[inline(always)]
fn mousebtn2code(mb: MouseButton) -> i32 {
    (mb as i32) - 1000
}
