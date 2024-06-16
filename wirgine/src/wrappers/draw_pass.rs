use crate::{
    c_functions::{
        wg_destroy_draw_pass, wg_draw_pass_create_on_finish_semaphore, wg_draw_pass_get_command,
        wg_draw_pass_render, wg_draw_pass_set_wait_semaphores,
    },
    c_types::{CDrawPass, CDrawPassSettings, CSemaphore},
};

use super::{command::Command, semaphore::Semaphore};

pub type DrawPassSettings = CDrawPassSettings;

pub struct DrawPass {
    draw_pass: CDrawPass,
}

impl DrawPass {
    pub fn new(draw_pass: CDrawPass) -> Self {
        Self { draw_pass }
    }

    pub fn get_command(&self) -> Command {
        let command = unsafe { wg_draw_pass_get_command(self.draw_pass) };

        Command::new(command)
    }

    pub fn set_wait_semaphores(&self, semaphores: &Vec<&mut Semaphore>) -> () {
        let mut c_semaphores: Vec<CSemaphore> = semaphores
            .iter()
            .map(|semaphore| semaphore.get_semaphore())
            .collect();
        unsafe {
            wg_draw_pass_set_wait_semaphores(
                self.draw_pass,
                c_semaphores.len() as u32,
                c_semaphores[..].as_mut_ptr(),
            );
        }
    }

    pub fn create_on_finish_semaphore(&self) -> Semaphore {
        unsafe {
            Semaphore::new(wg_draw_pass_create_on_finish_semaphore(
                self.get_draw_pass(),
            ))
        }
    }

    pub fn render(&self) -> () {
        unsafe {
            wg_draw_pass_render(self.draw_pass);
        }
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
