use spurv_rs::{
    shader::{shader::VertexShader, FragmentShader},
    types::{matrices::Matrix4T, structs::SingleFieldStructT, Vec4T},
    Vec4,
};

use crate::wrappers::{
    shader::{Shader, ShaderStage},
    wingine::Wingine,
};

pub fn get_basic_vertex_shader(wing: &Wingine) -> Shader {
    let vertex_vec = {
        let mut vertex_shader = VertexShader::create_vertex_shader();

        let mut vertex_output = vertex_shader.get_output_position();
        let mut color_output = vertex_shader.get_output::<Vec4T>(0);

        let matrix_uniform = vertex_shader.get_uniform::<SingleFieldStructT<Matrix4T>>(0, 0);

        let transformed_position =
            &matrix_uniform.get_member().load() * &vertex_shader.get_input::<Vec4T>(0).load();

        *vertex_output = transformed_position;
        *color_output = vertex_shader.get_input::<Vec4T>(1).load();

        vertex_shader.compile()
    };

    wing.create_shader(ShaderStage::Vertex, &vertex_vec)
}

pub fn get_basic_fragment_shader(wing: &Wingine, width: u32) -> Shader {
    let fragment_vec = {
        let mut fragment_shader = FragmentShader::create_fragment_shader();

        let mut color_output = fragment_shader.get_output::<Vec4T>(0);

        let color = Vec4::from_elements(1, 1, 0, 1);
        let color2 = Vec4::from_elements(0, 1, 0, 1);

        let coords = &(*fragment_shader.get_frag_coords());

        *color_output = color;

        fragment_shader.if_then(
            &coords.swizzle2(0, 1).length().greater_than(width * 7 / 8),
            |_| {
                *color_output = color2.clone();
            },
        );

        fragment_shader.if_then(&coords.at(1).less_than(width / 2), |shader| {
            *color_output = shader.get_input::<Vec4T>(0).load()
        });

        fragment_shader.compile()
    };

    wing.create_shader(ShaderStage::Fragment, &fragment_vec)
}
