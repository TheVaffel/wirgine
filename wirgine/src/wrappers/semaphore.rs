use crate::{c_functions::wg_destroy_semaphore, c_types::CSemaphore};

pub struct Semaphore {
    semaphore: CSemaphore,
}

impl Semaphore {
    pub fn new(semaphore: CSemaphore) -> Self {
        Semaphore { semaphore }
    }

    pub fn get_semaphore(&self) -> CSemaphore {
        self.semaphore
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) -> () {
        unsafe {
            wg_destroy_semaphore(self.semaphore);
        }
    }
}
