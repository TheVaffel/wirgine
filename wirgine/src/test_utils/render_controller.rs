use crate::wrappers::{wingine::Wingine, winval::Winval};

use super::{image::Image, image_test::create_or_compare_images};

use std::env;

pub type RenderFunctionType = fn(&mut Wingine) -> ();

pub trait RenderControllerTrait {
    fn get_wing(&mut self) -> &mut Wingine;

    fn render_loop(&mut self, render_function: &mut impl FnMut(&mut Wingine) -> ()) -> ();

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
}

pub enum RenderController {
    WindowController(WindowRenderController),
    TestController(TestRenderController),
}

impl RenderControllerTrait for RenderController {
    fn get_wing(&mut self) -> &mut Wingine {
        match self {
            RenderController::WindowController(controller) => controller.get_wing(),
            RenderController::TestController(controller) => controller.get_wing(),
        }
    }

    fn render_loop(&mut self, render_function: &mut impl FnMut(&mut Wingine) -> ()) -> () {
        match self {
            RenderController::WindowController(controller) => {
                controller.render_loop(render_function)
            }
            RenderController::TestController(controller) => controller.render_loop(render_function),
        }
    }

    fn get_width(&self) -> u32 {
        match self {
            RenderController::WindowController(controller) => controller.get_width(),
            RenderController::TestController(controller) => controller.get_width(),
        }
    }

    fn get_height(&self) -> u32 {
        match self {
            RenderController::WindowController(controller) => controller.get_height(),
            RenderController::TestController(controller) => controller.get_height(),
        }
    }
}

pub struct WindowRenderController {
    win: Winval,
    wing: Wingine,
    name: String,
    width: u32,
    height: u32,
}

impl WindowRenderController {
    pub fn new(name: &String) -> Self {
        let width = 800;
        let height = 800;
        let win = Winval::new(width, height);
        let wing = Wingine::with_winval(&win, name);

        Self {
            win,
            wing,
            name: name.clone(),
            width,
            height,
        }
    }
}

impl RenderControllerTrait for WindowRenderController {
    fn get_wing<'a>(&'a mut self) -> &'a mut Wingine {
        &mut self.wing
    }

    fn render_loop(&mut self, render_function: &mut impl FnMut(&mut Wingine) -> ()) -> () {
        while self.win.is_window_open() {
            render_function(&mut self.wing);

            self.win.sleep_milliseconds(40);

            self.win.flush_events();

            if self.win.is_key_pressed(0xFF1B) {
                // 0xFF1B = XK_Escape
                break;
            }
        }

        self.wing.wait_idle();
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}

pub struct TestRenderController {
    wing: Wingine,
    name: String,
    width: u32,
    height: u32,
}

impl TestRenderController {
    pub fn new(name: &String) -> Self {
        let width = 200;
        let height = 200;

        let wing = Wingine::new_headless(width, height, name);
        Self {
            wing,
            name: name.clone(),
            width,
            height,
        }
    }
}

impl RenderControllerTrait for TestRenderController {
    fn get_wing<'a>(&'a mut self) -> &'a mut Wingine {
        &mut self.wing
    }

    fn render_loop(&mut self, render_function: &mut impl FnMut(&mut Wingine) -> ()) -> () {
        let images: Vec<Image<u32>> = (0..3)
            .map(|_| {
                render_function(&mut self.wing);

                self.wing.get_last_rendered_image()
            })
            .collect();

        let result = create_or_compare_images(&self.name, &images);
        self.wing.wait_idle();
        result.expect("Test failed");
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}

pub fn create_render_controller(name: &String) -> RenderController {
    let run_windowed = env::var("WINDOWED").is_ok();

    if run_windowed {
        return RenderController::WindowController(WindowRenderController::new(name));
    } else {
        return RenderController::TestController(TestRenderController::new(name));
    }
}
