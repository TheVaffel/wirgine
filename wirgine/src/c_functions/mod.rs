use crate::c_types::*;

use libc::{c_char, c_uint, c_void};

pub mod winval;

#[link(name = "wingine_c")]
extern "C" {

    pub fn wg_create_wingine_headless(width: u32, height: u32, app_name: *const c_char)
        -> CWingine;

    pub fn wg_create_wingine_with_winval(win: CWinval, app_name: *const c_char) -> CWingine;

    pub fn wg_create_wingine_with_handles(
        width: u32,
        height: u32,
        handle_0: *const c_void, // In X: Window, in Windows: HINSTANCE
        handle_1: *const c_void, // In X: Display, in Windows: HWND
        app_name: *const c_char,
    ) -> CWingine;

    pub fn wg_create_vertex_buffer(wing: CWingine, size: u32) -> CVertexBuffer;
    pub fn wg_set_vertex_buffer(
        buffer: CVertexBuffer,
        data: *const c_void,
        byte_offset: u32,
        byte_size: u32,
    );
    pub fn wg_create_index_buffer(wing: CWingine, size: u32) -> CIndexBuffer;
    pub fn wg_set_index_buffer(
        buffer: CIndexBuffer,
        indices: *const u32,
        index_offset: u32,
        index_count: u32,
    );

    pub fn wg_create_uniform(wing: CWingine, size: u32) -> CUniform;

    pub fn wg_read_spv(file_name: *const c_char, spurv_words: *mut u32) -> *mut u32;

    pub fn wg_create_shader(
        wing: CWingine,
        shader_stage: CShaderStage,
        spv: *const u32,
        num_words: u32,
    ) -> CShader;

    pub fn wg_create_pipeline(
        wing: CWingine,
        num_attribs: u32,
        attrib_descs: *const CVertexAttribDesc,
        num_shaders: u32,
        shaders: *const CShader,
    ) -> CPipeline;

    pub fn wg_create_draw_pass(
        wing: CWingine,
        pipeline: CPipeline,
        draw_pass_settings: CDrawPassSettings,
    ) -> CDrawPass;
    pub fn wg_draw_pass_get_command(draw_pass: CDrawPass) -> CCommand;

    pub fn wg_get_default_framebuffer(wing: CWingine) -> CFramebuffer;

    pub fn wg_cmd_start_recording(command: CCommand, framebuffer: CFramebuffer);
    pub fn wg_cmd_bind_resource_set(
        comand: CCommand,
        resource_set: u32,
        num_bindings: u32,
        bindings: *const CResourceBinding,
    );
    pub fn wg_cmd_draw(
        command: CCommand,
        num_buffers: u32,
        vertex_buffers: *const CVertexBuffer,
        index_buffer: CIndexBuffer,
    );
    pub fn wg_cmd_end_recording(command: CCommand);

    pub fn wg_draw_pass_set_wait_semaphores(
        draw_pass: CDrawPass,
        num_semaphores: u32,
        semaphores: *mut CSemaphore,
    );

    pub fn wg_draw_pass_create_on_finish_semaphore(draw_pass: CDrawPass) -> CSemaphore;
    pub fn wg_wingine_set_present_wait_semaphores(
        wing: CWingine,
        num_semaphores: u32,
        semaphores: *mut CSemaphore,
    );

    pub fn wg_wingine_is_window_open(wing: CWingine) -> u8;

    pub fn wg_uniform_set_current(uniform: CUniform, data: *const c_void);
    pub fn wg_draw_pass_render(draw_pass: CDrawPass);
    pub fn wg_wingine_present(wing: CWingine);
    pub fn wg_wingine_sleep_milliseconds(wing: CWingine, millis: u32);
    pub fn wg_wingine_flush_events(wing: CWingine);

    pub fn wg_wingine_get_window_width(wing: CWingine) -> u32;
    pub fn wg_wingine_get_window_height(wing: CWingine) -> u32;

    pub fn wg_wingine_copy_last_rendered_image(wing: CWingine, dest: *mut u32);

    pub fn wg_wingine_is_key_pressed(wing: CWingine, x11_keycode: c_uint) -> u8;
    pub fn wg_wingine_wait_idle(wing: CWingine);

    pub fn wg_destroy_semaphore(semaphore: CSemaphore);
    pub fn wg_destroy_draw_pass(draw_pass: CDrawPass);
    pub fn wg_destroy_pipeline(pipeline: CPipeline);
    pub fn wg_destroy_shader(shader: CShader);

    pub fn wg_free_spv(spv: *mut u32);

    pub fn wg_destroy_uniform(uniform: CUniform);
    pub fn wg_destroy_index_buffer(buffer: CIndexBuffer);

    pub fn wg_destroy_vertex_buffer(vertex_buffer: CVertexBuffer);
    pub fn wg_destroy_wingine(wing: CWingine);
}
