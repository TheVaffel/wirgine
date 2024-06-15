use crate::{c_types::CWingine, c_functions::{wg_create_wingine, wg_destroy_wingine, wg_create_vertex_buffer, wg_create_index_buffer, wg_create_uniform, wg_create_shader}};

use super::{vertex_buffer::VertexBuffer, index_buffer::IndexBuffer, uniform::Uniform, shader::{Shader, ShaderStage}};

use crate::utils::IsReprC;

use std::mem::size_of;
use std::marker::PhantomData;

pub struct Wingine {
    wingine: CWingine
}

impl Wingine {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe {
            Self {
                wingine: wg_create_wingine(width, height)
            }
        }
    }

    pub fn create_vertex_buffer<T>(&self, num_elements: u32) -> VertexBuffer<T> {
        let type_size = size_of::<T>();
        unsafe {
            VertexBuffer::<T>::new(
                wg_create_vertex_buffer(self.wingine, num_elements * type_size as u32)
            )
        }
    }

    pub fn create_index_buffer(&self, num_elements: u32) -> IndexBuffer {
        let type_size = size_of::<u32>();
        unsafe {
            IndexBuffer::new(
                wg_create_index_buffer(self.wingine, num_elements * type_size as u32)
            )
        }
    }

    pub fn create_uniform<T: IsReprC>(&self) -> Uniform<T> {
        let type_size = size_of::<T>();
        unsafe {
            Uniform::<T>::new(
                wg_create_uniform(self.wingine, type_size as u32)
            )
        }
    }

    pub fn create_shader(&self, shader_stage: ShaderStage, bytecode: &Vec<u32>) -> Shader {
        let bytecode_ptr = bytecode[..].as_ptr();
        unsafe {
            Shader::new(wg_create_shader(self.wingine, shader_stage, bytecode_ptr, bytecode.len() as u32))
        }
    }

    pub fn get_wingine(&self) -> CWingine {
        self.wingine
    }
}

impl Drop for Wingine {
    fn drop(&mut self) -> () {
        unsafe { wg_destroy_wingine(self.wingine) };
    }
}
