#![feature(extern_types)]

mod c_functions;
mod c_types;
mod utils;
mod wrappers;

mod test_utils;

extern crate wirgine_macros;

use utils::IsReprC;
use wirgine_macros::IsReprC;

#[derive(IsReprC)]
#[repr(C)]
struct MatrixStruct {
    mat: [f32; 16],
}

#[cfg(test)]
mod tests {
    use crate::c_types::*;
    use crate::test_utils::image::Image;
    use crate::test_utils::image_test::create_or_compare_images;
    use crate::test_utils::render_controller::{
        RenderController, RenderControllerTrait, TestRenderController, WindowRenderController,
    };
    use crate::wrappers::draw_pass::{self, DrawPassSettings};
    use crate::wrappers::resource::ResourceBinding;
    use crate::wrappers::shader::{Shader, ShaderStage};
    use crate::wrappers::vertex_attrib_desc::{ComponentType, VertexAttribDesc};
    use crate::wrappers::vertex_buffer::GenericVertexBuffer;
    use crate::wrappers::wingine::Wingine;
    use crate::wrappers::winval::Winval;

    use super::c_functions::*;

    use std::mem::size_of;

    use libc::c_void;

    use spurv_rs::shader::shader::VertexShader;
    use spurv_rs::shader::FragmentShader;
    use spurv_rs::types::matrices::Matrix4T;
    use spurv_rs::types::structs::SingleFieldStructT;
    use spurv_rs::types::Vec4T;
    use spurv_rs::values::matrix::Matrix4;
    use spurv_rs::Vec4;

    use std::fmt::format;
    use std::path::Path;
    use std::rc::Rc;

    use core::array::from_fn;

    use crate::MatrixStruct;

    #[test]
    fn triangle() {
        let mut render_controller = RenderController::WindowController(
            WindowRenderController::new(&String::from("simple_test")),
        );

        const NUM_POINTS: usize = 3;
        const NUM_TRIANGLES: usize = 1;

        let position: [f32; NUM_POINTS * 4] = [
            -1.0, -1.0, -2.0, 1.0, 0.0, 1.0, -2.0, 1.0, 1.0, -1.0, -2.0, 1.0,
        ];

        let colors: [f32; NUM_POINTS * 4] =
            [0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0];

        let indices: [u32; NUM_TRIANGLES * 3] = [0, 1, 2];

        let wing = render_controller.get_wing();

        let mut position_buffer = wing.create_vertex_buffer::<f32>(NUM_POINTS as u32 * 4);
        position_buffer.set(&position[..]);

        let mut color_buffer = wing.create_vertex_buffer::<f32>(NUM_POINTS as u32 * 4);
        color_buffer.set(&colors[..]);

        let mut index_buffer = wing.create_index_buffer(NUM_TRIANGLES as u32 * 3);
        index_buffer.set(&indices[..]);

        let camera_uniform = wing.create_uniform::<MatrixStruct>();

        let attrib_descs = vec![
            VertexAttribDesc::new(0, ComponentType::Float32, 4, 4 * size_of::<f32>() as u32, 0),
            VertexAttribDesc::new(1, ComponentType::Float32, 4, 4 * size_of::<f32>() as u32, 0),
        ];

        let vertex_vec = {
            let mut vertex_shader = VertexShader::create_vertex_shader();

            let mut vertex_output = vertex_shader.get_output_position();
            let mut color_output = vertex_shader.get_output::<Vec4T>(0);

            let matrix_uniform = vertex_shader.get_uniform::<SingleFieldStructT<Matrix4T>>(0, 0);

            /* let matrix = Matrix4::from_columns(&Vec4::from_elements(1.0, 0.0, 0.0, 0.0),
            &Vec4::from_elements(0.0, 1.0, 0.0, 0.0),
            &Vec4::from_elements(0.0, 0.0, 1.0, 0.0),
            &Vec4::from_elements(0.0, 0.0, 0.0, 1.0));

            let transformed_position = vertex_shader.get_input::<Vec4T>(0).load(); */
            let transformed_position =
                &matrix_uniform.get_member().load() * &vertex_shader.get_input::<Vec4T>(0).load();

            *vertex_output = transformed_position;
            *color_output = vertex_shader.get_input::<Vec4T>(1).load();

            vertex_shader.compile()
        };
        println!("Length: {}", vertex_vec.len());
        for u in &vertex_vec {
            println!("{}", u);
        }
        let fragment_vec = {
            let mut fragment_shader = FragmentShader::create_fragment_shader();

            let mut color_output = fragment_shader.get_output::<Vec4T>(0);

            let color = Vec4::from_elements(1, 1, 0, 1);
            let color2 = Vec4::from_elements(0, 1, 0, 1);

            let coords = &(*fragment_shader.get_frag_coords());

            *color_output = color;

            fragment_shader.if_then(
                &coords
                    .swizzle2(0, 1)
                    .length()
                    .greater_than(render_controller.get_width() * 7 / 8),
                |_| {
                    *color_output = color2.clone();
                },
            );

            fragment_shader.if_then(
                &coords.at(1).less_than(render_controller.get_width() / 2),
                |shader| *color_output = shader.get_input::<Vec4T>(0).load(),
            );

            fragment_shader.compile()
        };

        let wing = render_controller.get_wing();

        let vertex_shader = wing.create_shader(ShaderStage::Vertex, &vertex_vec);
        let fragment_shader = wing.create_shader(ShaderStage::Fragment, &fragment_vec);

        let shaders = vec![&vertex_shader, &fragment_shader];

        let pipeline = wing.create_pipeline(&attrib_descs, &shaders);

        let mut draw_pass_settings = DrawPassSettings::default();
        draw_pass_settings.render_pass_settings.should_clear_color = 1;
        draw_pass_settings.render_pass_settings.should_clear_depth = 1;

        let draw_pass = wing.create_draw_pass(&pipeline, draw_pass_settings);

        let command = draw_pass.get_command();
        let vertex_buffers: Vec<&dyn GenericVertexBuffer> = vec![&position_buffer, &color_buffer];

        let bindings = vec![ResourceBinding::new(0, &camera_uniform)];

        command.start_recording(&wing.get_default_framebuffer());
        command.bind_resource_set(0, &bindings);
        command.draw(&vertex_buffers, &index_buffer);
        command.end_recording();

        let mut image_ready_semaphore = wing.create_image_ready_semaphore();
        draw_pass.set_wait_semaphores(&vec![&mut image_ready_semaphore]);

        let mut on_finish_semaphore = draw_pass.create_on_finish_semaphore();
        wing.set_present_wait_semaphores(&vec![&mut on_finish_semaphore]);

        // column-major
        let camera_struct: MatrixStruct = MatrixStruct {
            mat: [
                1.73205, 0.0, 0.0, 0.0, 0.0, -1.5396, 0.0, 0.0, 0.0, 0.0, -1.0001, -1.0, 0.0, 0.0,
                -0.010001, 0.0,
            ],
        };

        // while win.is_window_open() {

        render_controller.render_loop(&mut |wing: &mut Wingine| {
            camera_uniform.set_current(&camera_struct);

            draw_pass.render();
            wing.present();
        });
    }
}
