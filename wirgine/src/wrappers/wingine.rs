use crate::{c_types::{CWingine, CShader, CVertexAttribDesc}, c_functions::{wg_create_wingine, wg_destroy_wingine, wg_create_vertex_buffer, wg_create_index_buffer, wg_create_uniform, wg_create_shader, wg_create_pipeline, wg_create_draw_pass, wg_get_default_framebuffer}};

use super::{vertex_buffer::VertexBuffer, index_buffer::IndexBuffer, uniform::Uniform, shader::{Shader, ShaderStage}, pipeline::Pipeline, vertex_attrib_desc::VertexAttribDesc, draw_pass::{DrawPassSettings, DrawPass}, framebuffer::Framebuffer};

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

    pub fn create_pipeline(&self, attrib_descs: &Vec<VertexAttribDesc>, shaders: &Vec<&Shader>) -> Pipeline {
        let c_attrib_descs: Vec<CVertexAttribDesc> = attrib_descs.iter().map(|desc| desc.get_attrib_desc()).collect();
        let c_shaders: Vec<CShader> = shaders.iter().map(|shader| shader.get_shader()).collect();
        unsafe {
            Pipeline::new(wg_create_pipeline(self.wingine, c_attrib_descs.len() as u32, c_attrib_descs[..].as_ptr(), c_shaders.len() as u32, c_shaders[..].as_ptr()))
        }
    }

    pub fn create_draw_pass(&self, pipeline: &Pipeline, draw_pass_settings: DrawPassSettings) -> DrawPass {
        unsafe {
            DrawPass::new(wg_create_draw_pass(self.get_wingine(), pipeline.get_pipeline(), draw_pass_settings))
        }
    }

    pub fn get_default_framebuffer(&self) -> Framebuffer {
        unsafe {
            Framebuffer::new(wg_get_default_framebuffer(self.get_wingine()))
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
