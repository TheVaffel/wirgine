use crate::{
    c_functions::{
        wg_create_draw_pass, wg_create_index_buffer, wg_create_pipeline, wg_create_shader,
        wg_create_uniform, wg_create_vertex_buffer, wg_create_wingine_headless,
        wg_create_wingine_with_handles, wg_create_wingine_with_winval, wg_destroy_wingine,
        wg_get_default_framebuffer, wg_wingine_copy_last_rendered_image,
        wg_wingine_create_image_ready_semaphore, wg_wingine_get_window_height,
        wg_wingine_get_window_width, wg_wingine_present, wg_wingine_set_present_wait_semaphores,
        wg_wingine_wait_idle,
    },
    c_types::{CSemaphore, CShader, CVertexAttribDesc, CWingine},
    test_utils::image::Image,
};

use super::{
    draw_pass::{DrawPass, DrawPassSettings},
    framebuffer::Framebuffer,
    index_buffer::IndexBuffer,
    pipeline::Pipeline,
    semaphore::Semaphore,
    shader::{Shader, ShaderStage},
    uniform::Uniform,
    vertex_attrib_desc::VertexAttribDesc,
    vertex_buffer::VertexBuffer,
    winval::Winval,
};

use crate::utils::IsReprC;
use std::ffi::{c_void, CString};

use std::mem::size_of;

pub struct Wingine {
    wingine: CWingine,
}

impl Wingine {
    pub fn new_headless(width: u32, height: u32, app_name: &str) -> Self {
        unsafe {
            Self {
                wingine: wg_create_wingine_headless(
                    width,
                    height,
                    CString::new(app_name).expect("Invalid app name").as_ptr(),
                ),
            }
        }
    }

    pub fn with_winval(winval: &Winval, app_name: &str) -> Self {
        unsafe {
            Self {
                wingine: wg_create_wingine_with_winval(
                    winval.get_winval(),
                    CString::new(app_name).expect("Invalid app name").as_ptr(),
                ),
            }
        }
    }

    pub fn with_handles(
        width: u32,
        height: u32,
        handle_0: *const (), // In X: Window, in Windows: HINSTANCE
        handle_1: *const (), // In X: Display, in Windows: HWND
        app_name: &str,
    ) -> Self {
        unsafe {
            Self {
                wingine: wg_create_wingine_with_handles(
                    width,
                    height,
                    handle_0 as *const c_void,
                    handle_1 as *const c_void,
                    CString::new(app_name).expect("Invalid app name").as_ptr(),
                ),
            }
        }
    }

    pub fn create_vertex_buffer<T>(&self, num_elements: usize) -> VertexBuffer<T> {
        let type_size = size_of::<T>();
        unsafe {
            VertexBuffer::<T>::new(wg_create_vertex_buffer(
                self.wingine,
                (num_elements * type_size) as u32,
            ))
        }
    }

    pub fn create_index_buffer(&self, num_elements: usize) -> IndexBuffer {
        let type_size = size_of::<u32>();
        unsafe {
            IndexBuffer::new(wg_create_index_buffer(
                self.wingine,
                (num_elements * type_size) as u32,
            ))
        }
    }

    pub fn create_uniform<T: IsReprC>(&self) -> Uniform<T> {
        let type_size = size_of::<T>();
        unsafe { Uniform::<T>::new(wg_create_uniform(self.wingine, type_size as u32)) }
    }

    pub fn create_shader(&self, shader_stage: ShaderStage, bytecode: &Vec<u32>) -> Shader {
        let bytecode_ptr = bytecode[..].as_ptr();
        unsafe {
            Shader::new(wg_create_shader(
                self.wingine,
                shader_stage,
                bytecode_ptr,
                bytecode.len() as u32,
            ))
        }
    }

    pub fn create_pipeline(
        &self,
        attrib_descs: &Vec<VertexAttribDesc>,
        shaders: &Vec<&Shader>,
    ) -> Pipeline {
        let c_attrib_descs: Vec<CVertexAttribDesc> = attrib_descs
            .iter()
            .map(|desc| desc.get_attrib_desc())
            .collect();
        let c_shaders: Vec<CShader> = shaders.iter().map(|shader| shader.get_shader()).collect();
        unsafe {
            Pipeline::new(wg_create_pipeline(
                self.wingine,
                c_attrib_descs.len() as u32,
                c_attrib_descs[..].as_ptr(),
                c_shaders.len() as u32,
                c_shaders[..].as_ptr(),
            ))
        }
    }

    pub fn create_draw_pass(
        &self,
        pipeline: &Pipeline,
        draw_pass_settings: DrawPassSettings,
    ) -> DrawPass {
        unsafe {
            DrawPass::new(wg_create_draw_pass(
                self.get_wingine(),
                pipeline.get_pipeline(),
                draw_pass_settings,
            ))
        }
    }

    pub fn create_image_ready_semaphore(&self) -> Semaphore {
        unsafe { Semaphore::new(wg_wingine_create_image_ready_semaphore(self.get_wingine())) }
    }

    pub fn get_default_framebuffer(&self) -> Framebuffer {
        unsafe { Framebuffer::new(wg_get_default_framebuffer(self.get_wingine())) }
    }

    pub fn set_present_wait_semaphores(&self, semaphores: &Vec<&mut Semaphore>) -> () {
        let mut c_semaphores: Vec<CSemaphore> = semaphores
            .iter()
            .map(|semaphore| semaphore.get_semaphore())
            .collect();
        unsafe {
            wg_wingine_set_present_wait_semaphores(
                self.get_wingine(),
                c_semaphores.len() as u32,
                c_semaphores[..].as_mut_ptr(),
            );
        }
    }

    pub fn present(&self) -> () {
        unsafe {
            wg_wingine_present(self.wingine);
        }
    }

    pub fn wait_idle(&self) -> () {
        unsafe {
            wg_wingine_wait_idle(self.wingine);
        }
    }

    pub fn get_last_rendered_image(&self) -> Image<u32> {
        unsafe {
            let width = wg_wingine_get_window_width(self.wingine) as u32;
            let height = wg_wingine_get_window_height(self.wingine) as u32;
            let mut image = Image::<u32>::new(width, height);
            wg_wingine_copy_last_rendered_image(self.wingine, image.data_ptr() as *mut u32);
            image
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
