use spurv_rs::{shader::shader::VertexShader, types::Vec4T};

use wirgine::{
    draw_pass::DrawPassSettings,
    shader::Shader,
    shader::ShaderStage,
    vertex_attrib_desc::{ComponentType, VertexAttribDesc},
    vertex_buffer::GenericVertexBuffer,
    wingine::Wingine,
};

use crate::render_controller::RenderControllerTrait;

use super::render_controller::RenderController;

pub fn test_fragment_shader(
    fragment_shader: Shader,
    render_controller: &mut RenderController,
) -> () {
    const NUM_POINTS: usize = 4;
    const NUM_TRIANGLES: usize = 2;

    let position: [f32; NUM_POINTS * 4] = [
        -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 0.0, 1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    ];

    let indices: [u32; NUM_TRIANGLES * 3] = [0, 1, 2, 2, 1, 3];

    let wing = render_controller.get_wing();

    let mut position_buffer = wing.create_vertex_buffer::<f32>(NUM_POINTS * 4);
    position_buffer.set(&position[..]);

    let mut index_buffer = wing.create_index_buffer(NUM_TRIANGLES * 3);
    index_buffer.set(&indices[..]);

    let attrib_descs = vec![VertexAttribDesc::new(
        0,
        ComponentType::Float32,
        4,
        4 * size_of::<f32>() as u32,
        0,
    )];

    let vertex_shader = identity_vertex_shader(wing);

    let wing = render_controller.get_wing();

    let shaders = vec![&vertex_shader, &fragment_shader];

    let pipeline = wing.create_pipeline(&attrib_descs, &shaders);

    let mut draw_pass_settings = DrawPassSettings::default();
    draw_pass_settings.render_pass_settings.should_clear_color = 1;
    draw_pass_settings.render_pass_settings.should_clear_depth = 1;

    let draw_pass = wing.create_draw_pass(&pipeline, draw_pass_settings);

    let vertex_buffers: Vec<&dyn GenericVertexBuffer> = vec![&position_buffer];

    let command = draw_pass.get_command();
    command.start_recording(&wing.get_default_framebuffer());
    command.draw(&vertex_buffers, &index_buffer);
    command.end_recording();
    let mut on_finish_semaphore = draw_pass.create_on_finish_semaphore();
    wing.set_present_wait_semaphores(&vec![&mut on_finish_semaphore]);

    render_controller.render_loop(&mut |wing: &mut Wingine| {
        draw_pass.render();
        wing.present();
    });
}

fn identity_vertex_shader(wing: &Wingine) -> Shader {
    let vertex_vec = {
        let mut vertex_shader = VertexShader::create_vertex_shader();

        let mut vertex_output = vertex_shader.get_output_position();

        *vertex_output = vertex_shader.get_input::<Vec4T>(0).load();

        vertex_shader.compile()
    };

    wing.create_shader(ShaderStage::Vertex, &vertex_vec)
}
