use crate::{
    c_functions::wg_destroy_shader,
    c_types::{CShader, CShaderStage},
};

pub type ShaderStage = CShaderStage;

pub struct Shader {
    shader: CShader,
}

impl Shader {
    pub fn new(shader: CShader) -> Self {
        Shader { shader }
    }

    pub fn get_shader(&self) -> CShader {
        self.shader
    }
}

impl Drop for Shader {
    fn drop(&mut self) -> () {
        unsafe {
            wg_destroy_shader(self.shader);
        }
    }
}
