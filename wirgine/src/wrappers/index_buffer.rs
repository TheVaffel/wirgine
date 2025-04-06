use crate::{
    c_functions::{wg_destroy_index_buffer, wg_set_index_buffer},
    c_types::CIndexBuffer,
};

pub struct IndexBuffer {
    index_buffer: CIndexBuffer,
}

impl IndexBuffer {
    pub fn new(index_buffer: CIndexBuffer) -> Self {
        Self { index_buffer }
    }

    pub fn set(&mut self, data: &[u32]) -> () {
        unsafe {
            wg_set_index_buffer(
                self.index_buffer,
                data.as_ptr() as *const u32,
                0,
                (data.len()) as u32,
            );
        }
    }

    pub fn set_with_offset(&mut self, data: &[u32], element_offset: u32) -> () {
        unsafe {
            wg_set_index_buffer(
                self.index_buffer,
                data.as_ptr() as *const u32,
                element_offset as u32,
                (data.len()) as u32,
            );
        }
    }

    pub fn get_index_buffer(&self) -> CIndexBuffer {
        self.index_buffer
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) -> () {
        unsafe {
            wg_destroy_index_buffer(self.index_buffer);
        }
    }
}
