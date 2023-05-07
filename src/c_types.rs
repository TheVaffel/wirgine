
mod attrib_desc;
mod draw_pass_settings;
mod resource_binding;

pub use attrib_desc::{CVertexAttribDesc,CComponentType};
pub use draw_pass_settings::CDrawPassSettings;
pub use resource_binding::CResourceBinding;

/*
 * Imported types
 */
extern "C" {
    pub type wg_wingine_t;
    pub type wg_vertex_buffer_t;
    pub type wg_index_buffer_t;
    pub type wg_uniform_t;
    pub type wg_vertex_attrib_desc_t;
    pub type wg_shader_t;
    pub type wg_pipeline_t;
    pub type wg_draw_pass_settings_t;
    pub type wg_draw_pass_t;
    pub type wg_command_t;
    pub type wg_framebuffer_t;
    pub type wg_resource_binding_t;
    pub type wg_semaphore_t;
}

#[repr(C)]
pub enum CShaderStage {
    Vertex = 0,
    Fragment = 1,
    Compute = 2
}

/*
 * Rustified types
 */
pub type CWingine = *mut wg_wingine_t;
pub type CVertexBuffer = *mut wg_vertex_buffer_t;
pub type CIndexBuffer = *mut wg_index_buffer_t;
pub type CUniform = *mut wg_uniform_t;
pub type CShader = *mut wg_shader_t;
pub type CPipeline = *mut wg_pipeline_t;
pub type CDrawPass = *mut wg_draw_pass_t;
pub type CCommand = *mut wg_command_t;
pub type CFramebuffer = *mut wg_framebuffer_t;
pub type CSemaphore = *mut wg_semaphore_t;
