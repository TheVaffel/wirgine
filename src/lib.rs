#![feature(extern_types)]

mod c_types;
mod c_functions;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::c_types::*;

    use super::c_functions::*;

    use std::mem::size_of;
    use std::ffi::CString;

    use libc::c_void;

    #[test]
    fn it_works() {
        unsafe {

            let width = 800;
            let height = 800;
            let num_points = 3;
            let num_triangles = 1;

            let mut position: [f32; 3 * 4] = [
                1.0, -1.0, -2.5, 1.0,
                -1.0, -1.0, -2.5, 1.0,
                0.0, 1.0, -2.5, 1.0,
            ];

            let mut colors: [f32; 3 * 4] = [
                0.0, 1.0, 0.0, 1.0,
                1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 1.0,
            ];

            let mut indices: [u32; 1 * 3] = [
                0, 1, 2,
            ];

            let wing = wg_create_wingine(800, 800);

            let position_buffer = wg_create_vertex_buffer(wing, (num_points * size_of::<f32>() * 4) as u32);
            wg_set_vertex_buffer(position_buffer,
                                 position[..].as_mut_ptr() as *mut _ as *mut c_void,
                                 0,
                                 (num_points * size_of::<f32>() * 4) as u32);

            let color_buffer = wg_create_vertex_buffer(wing, (num_points * size_of::<f32>() * 4) as u32);
            wg_set_vertex_buffer(color_buffer,
                                 colors[..].as_mut_ptr() as *mut _ as *mut c_void,
                                 0,
                                 (num_points * size_of::<f32>() * 4) as u32);

            let vertex_buffers: [CVertexBuffer; 2] = [ position_buffer, color_buffer ];

            let index_buffer = wg_create_index_buffer(wing, num_triangles * 3);
            wg_set_index_buffer(index_buffer,
                                indices[..].as_mut_ptr(),
                                0,
                                num_triangles * 3);

            let camera_uniform = wg_create_uniform(wing, 16 * size_of::<f32>() as u32);

            let attrib_descs: [CVertexAttribDesc; 2] = [
                CVertexAttribDesc::new(0, CComponentType::Float32, 4, 4 * size_of::<f32>() as u32, 0),
                CVertexAttribDesc::new(1, CComponentType::Float32, 4, 4 * size_of::<f32>() as u32, 0)
            ];

            let mut vertex_words = 0u32;
            let mut fragment_words = 0u32;

            let vertex_file_name = CString::new("./vertex.spv").unwrap();
            let fragment_file_name = CString::new("./fragment.spv").unwrap();

            let vertex_spv = wg_read_spv(vertex_file_name.as_ptr(),
                                         &mut vertex_words as *mut u32);
            let fragment_spv = wg_read_spv(fragment_file_name.as_ptr(),
                                           &mut fragment_words as *mut u32);

            println!("Byte code lengths - vertex: {}, fragment: {}", vertex_words, fragment_words);

            let vertex_shader = wg_create_shader(wing, CShaderStage::Vertex, vertex_spv, vertex_words);
            let fragment_shader = wg_create_shader(wing, CShaderStage::Fragment, fragment_spv, fragment_words);

            let shaders: [CShader; 2] = [
                vertex_shader, fragment_shader
            ];

            let pipeline = wg_create_pipeline(wing, 2, attrib_descs[..].as_ptr(), 2, shaders[..].as_ptr());

            let mut draw_pass_settings = CDrawPassSettings::default();
            draw_pass_settings.render_pass_settings.should_clear_color = 1;
            draw_pass_settings.render_pass_settings.should_clear_depth = 1;

            let draw_pass = wg_create_draw_pass(wing, pipeline, draw_pass_settings);

            let command = wg_draw_pass_get_command(draw_pass);

            let bindings: [CResourceBinding; 1] = [ CResourceBinding { binding: 0, resource: camera_uniform as *mut _ as *mut c_void} ];

            wg_cmd_start_recording(command, wg_get_default_framebuffer(wing));
            wg_cmd_bind_resource_set(command, 0, 1, bindings[..].as_ptr());
            wg_cmd_draw(command, 2, vertex_buffers[..].as_ptr(), index_buffer);
            wg_cmd_end_recording(command);

            let image_ready_semaphore = wg_wingine_create_image_ready_semaphore(wing);
            wg_draw_pass_set_wait_semaphores(draw_pass, 1, [image_ready_semaphore][..].as_mut_ptr());

            let on_finish_semaphore = wg_draw_pass_create_on_finish_semaphore(draw_pass);
            wg_wingine_set_present_wait_semaphores(wing, 1, [on_finish_semaphore][..].as_mut_ptr());

            let camera_matrix: [f32; 16] = [ 1.73205, 0.0, 0.0, 0.0,
                                             0.0, -1.5396, 0.0, 0.0,
                                             0.0, 0.0, -1.0001, -1.0,
                                             0.0, 0.0, -0.010001, 0.0 ];

            while wg_wingine_is_window_open(wing) != 0 {
                wg_uniform_set_current(camera_uniform, camera_matrix[..].as_ptr() as *const _ as *const c_void);
                wg_draw_pass_render(draw_pass);
                wg_wingine_present(wing);
                wg_wingine_sleep_milliseconds(wing, 40);

                wg_wingine_flush_events(wing);

                if wg_wingine_is_key_pressed(wing, 0xFF1B) != 0 {
                    break;
                }
            }

            wg_wingine_wait_idle(wing);

            wg_destroy_semaphore(on_finish_semaphore);
            wg_destroy_semaphore(image_ready_semaphore);

            wg_destroy_draw_pass(draw_pass);

            wg_destroy_pipeline(pipeline);

            wg_destroy_shader(vertex_shader);
            wg_destroy_shader(fragment_shader);

            wg_free_spv(vertex_spv);
            wg_free_spv(fragment_spv);

            wg_destroy_uniform(camera_uniform);

            wg_destroy_vertex_buffer(position_buffer);
            wg_destroy_vertex_buffer(color_buffer);
            wg_destroy_index_buffer(index_buffer);
            wg_destroy_wingine(wing);
        }
    }
}
