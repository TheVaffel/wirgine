use std::mem::size_of;
use wirgine::{
    draw_pass::DrawPassSettings,
    resource::ResourceBinding,
    vertex_attrib_desc::{ComponentType, VertexAttribDesc},
    vertex_buffer::GenericVertexBuffer,
    wingine::Wingine,
    IsReprC, IsReprCMacro,
};

use test_utils::{
    render_controller::{create_render_controller, RenderControllerTrait},
    shaders::{get_basic_fragment_shader, get_basic_vertex_shader},
};

#[derive(IsReprCMacro)]
#[repr(C)]
struct MatrixStruct {
    mat: [f32; 16],
}

#[test]
fn triangle() {
    const NUM_POINTS: usize = 3;
    const NUM_TRIANGLES: usize = 1;

    let position: [f32; NUM_POINTS * 4] = [
        -1.0, -1.0, -2.0, 1.0, 0.0, 1.0, -2.0, 1.0, 1.0, -1.0, -2.0, 1.0,
    ];

    let colors: [f32; NUM_POINTS * 4] =
        [0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0];

    let indices: [u32; NUM_TRIANGLES * 3] = [0, 1, 2];

    let mut render_controller = create_render_controller(&String::from("simple_test"));
    let width = render_controller.get_width();

    let wing = render_controller.get_wing();

    let mut position_buffer = wing.create_vertex_buffer::<f32>(NUM_POINTS * 4);
    position_buffer.set(&position[..]);

    let mut color_buffer = wing.create_vertex_buffer::<f32>(NUM_POINTS * 4);
    color_buffer.set(&colors[..]);

    let mut index_buffer = wing.create_index_buffer(NUM_TRIANGLES * 3);
    index_buffer.set(&indices[..]);

    let camera_uniform = wing.create_uniform::<MatrixStruct>();

    let attrib_descs = vec![
        VertexAttribDesc::new(0, ComponentType::Float32, 4, 4 * size_of::<f32>() as u32, 0),
        VertexAttribDesc::new(1, ComponentType::Float32, 4, 4 * size_of::<f32>() as u32, 0),
    ];

    let vertex_shader = get_basic_vertex_shader(wing);
    let fragment_shader = get_basic_fragment_shader(wing, width);

    let wing = render_controller.get_wing();

    let shaders = vec![&vertex_shader, &fragment_shader];

    let pipeline = wing.create_pipeline(&attrib_descs, &shaders);

    let mut draw_pass_settings = DrawPassSettings::default();
    draw_pass_settings.render_pass_settings.should_clear_color = 1;
    draw_pass_settings.render_pass_settings.should_clear_depth = 1;

    let draw_pass = wing.create_draw_pass(&pipeline, draw_pass_settings);

    let vertex_buffers: Vec<&dyn GenericVertexBuffer> = vec![&position_buffer, &color_buffer];

    let resource_set = vec![ResourceBinding::new(0, &camera_uniform)];

    let command = draw_pass.get_command();
    command.start_recording(&wing.get_default_framebuffer());
    command.bind_resource_set(0, &resource_set);
    command.draw(&vertex_buffers, &index_buffer);
    command.end_recording();

    // column-major
    let camera_struct = MatrixStruct {
        mat: [
            1.73205, 0.0, 0.0, 0.0, 0.0, -1.5396, 0.0, 0.0, 0.0, 0.0, -1.0001, -1.0, 0.0, 0.0,
            -0.010001, 0.0,
        ],
    };

    render_controller.render_loop(&mut |wing: &mut Wingine| {
        camera_uniform.set_current(&camera_struct);

        draw_pass.render();
        wing.present();
    });
}
