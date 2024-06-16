use crate::c_types::{win_winval_t, CWinval};

#[link(name = "wingine_c")]
extern "C" {

    pub fn win_create_winval(width: u32, height: u32) -> CWinval;

    pub fn win_winval_is_window_open(win: CWinval) -> u8;
    pub fn win_winval_is_key_pressed(win: CWinval, key: u32) -> u8;
    pub fn win_winval_sleep_milliseconds(win: CWinval, millis: u32) -> ();
    pub fn win_winval_flush_events(win: CWinval) -> ();

    pub fn win_destroy_winval(win: CWinval) -> ();
}
