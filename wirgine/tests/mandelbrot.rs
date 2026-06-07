use spurv_rs::{shader::FragmentShader, types::Vec4T, Vec4};
#[cfg(test)]
#[test]
fn mandelbrot() -> () {
    use std::fs::File;
    use std::io::Write;

    use test_utils::{
        render_controller::{create_render_controller, RenderControllerTrait},
        test_fragment::test_fragment_shader,
    };
    use wirgine::shader::ShaderStage;

    let mut render_controller = create_render_controller(&String::from("Mandelbrot"));
    let width = render_controller.get_width();
    let height = render_controller.get_height();

    let fragment_vec = {
        use spurv_rs::Vec2;

        let mut fragment_shader = FragmentShader::create_fragment_shader();

        let mut color_output = fragment_shader.get_output::<Vec4T>(0);

        let coords = &(*fragment_shader.get_frag_coords());

        let center_x = -0.374;
        let center_y = -0.2;
        let zoom_level = 300;

        let zoom_middle = 1.0 / (2 * zoom_level) as f32;

        let normalized = Vec2::from_elements(
            &(&(&coords.at(0) / ((width * zoom_level) as f32)) - (zoom_middle + center_x)),
            &(&(&coords.at(1) / ((height * zoom_level) as f32)) - (zoom_middle + center_y)),
        );

        let current_point0 = fragment_shader.create_local(&normalized);
        let mut current_point1 = current_point0.clone();
        let iteration0 = fragment_shader.create_local(0.0);
        let mut iteration1 = iteration0.clone();

        fragment_shader.meanwhile(
            |_shader| {
                ((*current_point0).length().less_than(4.0)).and(&(*iteration0).less_than(1000.0))
            },
            |_shader| {
                let p = &*current_point1;
                let x = &(&p.at(0) * &p.at(0)) - &(&p.at(1) * &p.at(1));
                let y = &(&p.at(0) * &p.at(1)) * 2.0;
                *current_point1 = &Vec2::from_elements(&x, &y) + &normalized;
                *iteration1 = &*iteration1 + 1.0;
            },
        );

        *color_output = Vec4::from_elements(0, 0, 0, 1);

        fragment_shader.if_then(&(*iteration1).less_than(999.0), |_shader| {
            *color_output = Vec4::from_elements(
                &(&*iteration1 * 0.0523).sin(),
                &(&*iteration1 * 0.0234).cos(),
                &(&*iteration1 * 0.0112).sin(),
                1.0,
            );
        });

        fragment_shader.compile()
    };

    {
        let file_name = "mandelbrot.spv";
        let mut file = File::create(file_name).unwrap();

        let len = 4 * fragment_vec.len();
        let ptr = fragment_vec[..].as_ptr() as *const u8;
        let byte_array = unsafe { std::slice::from_raw_parts(ptr, len) };
        file.write_all(&byte_array).unwrap();
        println!("Wrote SPV to {}", file_name);
    }

    let shader = render_controller
        .get_wing()
        .create_shader(ShaderStage::Fragment, &fragment_vec);

    test_fragment_shader(shader, &mut render_controller);
}
