use crate::{
    c_functions::{wg_destroy_vertex_buffer, wg_set_vertex_buffer},
    c_types::CVertexBuffer,
};

use super::wingine::Wingine;

use libc::c_void;

use std::marker::PhantomData;
use std::mem::size_of;

pub struct VertexBuffer<T> {
    vertex_buffer: CVertexBuffer,
    _data: PhantomData<T>,
}

impl<T> VertexBuffer<T> {
    pub fn new(vertex_buffer: CVertexBuffer) -> Self {
        Self {
            vertex_buffer,
            _data: PhantomData,
        }
    }

    pub fn set(&mut self, data: &[T]) -> () {
        let type_size = size_of::<T>();
        unsafe {
            wg_set_vertex_buffer(
                self.vertex_buffer,
                data.as_ptr() as *const c_void,
                0,
                (type_size * data.len()) as u32,
            );
        }
    }

    pub fn set_with_offset(&mut self, data: &[T], element_offset: u32) -> () {
        let type_size = size_of::<T>();
        unsafe {
            wg_set_vertex_buffer(
                self.vertex_buffer,
                data.as_ptr() as *const c_void,
                element_offset * type_size as u32,
                (type_size * data.len()) as u32,
            );
        }
    }

    pub fn get_vertex_buffer(&self) -> CVertexBuffer {
        self.vertex_buffer
    }
}

impl<T> Drop for VertexBuffer<T> {
    fn drop(&mut self) -> () {
        unsafe { wg_destroy_vertex_buffer(self.vertex_buffer) };
    }
}

pub trait GenericVertexBuffer {
    fn get_vertex_buffer(&self) -> CVertexBuffer;
}

impl<T> GenericVertexBuffer for VertexBuffer<T> {
    fn get_vertex_buffer(&self) -> CVertexBuffer {
        self.get_vertex_buffer()
    }
}
