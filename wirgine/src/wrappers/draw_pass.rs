use crate::{c_types::{CDrawPass, CDrawPassSettings}, c_functions::{wg_destroy_draw_pass, wg_draw_pass_get_command}};

use super::command::Command;


pub type DrawPassSettings = CDrawPassSettings;

pub struct DrawPass {
    draw_pass: CDrawPass
}

impl DrawPass {
    pub fn new(draw_pass: CDrawPass) -> Self {
        Self {
            draw_pass
        }
    }

    pub fn get_command(&self) -> Command {
        let command = unsafe {
            wg_draw_pass_get_command(self.draw_pass)
        };

        Command::new(command)
    }

    pub fn get_draw_pass(&self) -> CDrawPass {
        self.draw_pass
    }
}

impl Drop for DrawPass {
    fn drop(&mut self) -> () {
        unsafe {
            wg_destroy_draw_pass(self.draw_pass);
        }
    }
}
