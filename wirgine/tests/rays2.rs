#![feature(trait_alias)]
use spurv_rs::{shader::FragmentShader, types::Vec4T, OutputVariable, Vec3, F32};

#[cfg(test)]
#[test]
fn rays() {
    use test_utils::{
        render_controller::{create_render_controller, RenderControllerTrait},
        test_fragment::test_fragment_shader,
    };
    use wirgine::shader::ShaderStage;

    let mut render_controller = create_render_controller(&String::from("rays2"));
    let width = render_controller.get_width();
    let height = render_controller.get_height();

    let fragment_vec = {
        use spurv_rs::{shader::FragmentShader, types::Vec4T};

        let mut fragment_shader = FragmentShader::create_fragment_shader();

        let mut color_output = fragment_shader.get_output::<Vec4T>(0);

        trace_sdf(&sdf, &mut color_output, width, height, &mut fragment_shader);

        fragment_shader.compile()
    };

    let shader = render_controller
        .get_wing()
        .create_shader(ShaderStage::Fragment, &fragment_vec);

    test_fragment_shader(shader, &mut render_controller);
}

trait SDF = Fn(&Vec3) -> F32;

fn trace_sdf(
    sdf: &impl SDF,
    output_color: &mut OutputVariable<Vec4T>,
    width: u32,
    height: u32,
    fragment_shader: &mut FragmentShader,
) -> () {
    use spurv_rs::{Vec3, Vec4};

    const MAX_ITERATIONS: u32 = 300;

    let coords = &(*fragment_shader.get_frag_coords());

    let origin = Vec3::from_elements(0, 0, 0);

    let direction = Vec3::from_elements(
        &(&(&coords.at(0) / ((width / 2) as f32)) - 1.0),
        &(&(&coords.at(1) / ((height / 2) as f32)) - 1.0),
        1.0,
    );

    let mut current_t = fragment_shader.create_local(0.0);
    let dist0 = fragment_shader.create_local(&sdf(&origin));
    let mut dist1 = dist0.clone();
    let counter0 = fragment_shader.create_local(0.0);
    let mut counter1 = counter0.clone();

    fragment_shader.meanwhile(
        |_| (&(*dist0).greater_than(0.01)).and(&(*counter0).less_than(MAX_ITERATIONS as f32)),
        |_| {
            let new_t = &(*current_t) + &*dist1;
            let new_p = &origin + &(direction.scale(&new_t));

            *current_t = new_t;
            *dist1 = sdf(&new_p);
            *counter1 = &(*counter1) + 1.0;
        },
    );

    let final_p = &origin + &(direction.scale(&*current_t));

    **output_color = Vec4::from_elements(0.2, 0.2, 0.2, 1.0);

    fragment_shader.if_then(&(*counter0).less_than(MAX_ITERATIONS as f32), |_| {
        let color = Vec3::from_elements(0.8, 0.5, 0.4)
            .scale(&-&get_sdf_normal(&final_p).dot(&Vec3::from_elements(-0.2, -0.5, 0.3)));
        **output_color = Vec4::from_elements(&color.at(0), &color.at(1), &color.at(2), 1.0);
    });
}

fn get_sdf_normal(p: &Vec3) -> Vec3 {
    let dist = sdf(p);
    let epsilon = 0.01;

    (&(&Vec3::from_elements(&dist, &dist, &dist)
        - &Vec3::from_elements(
            &sdf(&(p - &Vec3::from_elements(epsilon, 0.0, 0.0))),
            &sdf(&(p - &Vec3::from_elements(0.0, epsilon, 0.0))),
            &sdf(&(p - &Vec3::from_elements(0.0, 0.0, epsilon))),
        )))
        .normalize()
}

fn sdf(p: &Vec3) -> F32 {
    sphere([0.0, 0.0, 3.0], 1.0)(p)
    // return sphere([0.0, 0.0, 3.0], 1.0)(p).min(&sphere([0.3, 0.4, 3.5], 1.3)(p));
}

fn sphere(pos: [f32; 3], radius: f32) -> impl SDF {
    let [x, y, z] = pos;
    move |p: &Vec3| {
        let diff = p - &Vec3::from_elements(x, y, z);
        &diff.length() - radius
    }
}
