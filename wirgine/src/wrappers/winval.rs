use crate::{
    c_functions::{
        wg_wingine_flush_events, wg_wingine_is_key_pressed, wg_wingine_is_window_open,
        wg_wingine_sleep_milliseconds,
        winval::{
            win_create_winval, win_destroy_winval, win_winval_flush_events,
            win_winval_is_key_pressed, win_winval_is_window_open, win_winval_sleep_milliseconds,
        },
    },
    c_types::{win_winval_t, CWingine, CWinval},
};

pub struct Winval {
    winval: CWinval,
}

impl Winval {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe {
            Winval {
                winval: win_create_winval(width, height),
            }
        }
    }

    pub fn sleep_milliseconds(&self, milliseconds: u32) -> () {
        unsafe {
            win_winval_sleep_milliseconds(self.winval, 40);
        }
    }

    pub fn flush_events(&self) -> () {
        unsafe {
            win_winval_flush_events(self.winval);
        }
    }

    pub fn is_window_open(&self) -> bool {
        let val = unsafe { win_winval_is_window_open(self.winval) };

        val != 0
    }

    pub fn is_key_pressed(&self, key: u32) -> bool {
        let val = unsafe { win_winval_is_key_pressed(self.winval, key) };

        val != 0
    }

    pub fn get_winval(&self) -> CWinval {
        self.winval
    }
}

impl Drop for Winval {
    fn drop(&mut self) -> () {
        unsafe {
            win_destroy_winval(self.winval);
        }
    }
}
