use glfw::{Glfw};

use vulkano::instance::{ApplicationInfo, Version, Instance};

use vulkano_glfw as vg;
use vulkano_glfw::GlfwWindow;

use std::borrow::Cow;
use std::sync::Arc;

// import functions from previous parts
use ::triangle::setup::base_code::init_window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn app_main() {
    let mut app = HelloTriangleApplication::new();

    let result = app.run();
    match result {
        Ok(_) => println!("OK"),
        Err(_) => println!("ERROR")
    }
}

struct HelloTriangleApplication {
    glfw: Glfw,
    window: GlfwWindow,
    _instance: Arc<Instance>,
}

impl HelloTriangleApplication {
    fn run(&mut self) -> Result<(),()> {
        self.main_loop();
        self.cleanup();
        Ok(())
    }

    fn main_loop(&mut self) {
        while !self.window.should_close() {
            self.glfw.poll_events();
        }
    }

    fn cleanup(&mut self) {
        // destroy window and terminate is not needed
        // because it is hadled by the library
    }

    fn new() -> HelloTriangleApplication {
        // initWindow stuff is here
        let (glfw, window) = init_window(WIDTH, HEIGHT);

        // init Vulkan
        let instance = create_instance(&glfw);

        HelloTriangleApplication {
            glfw: glfw,
            window: window,
            _instance: instance,
        }
    }
}

fn create_instance(glfw: &Glfw) -> Arc<Instance> {
    // initVulkan stuff is here
    let mut app_info = ApplicationInfo::default();
    app_info.application_name = Some(Cow::Borrowed("Hello Triangle"));
    app_info.application_version = Some(Version { major: 1, minor: 0, patch: 0 });
    app_info.engine_name = Some(Cow::Borrowed("No Engine"));
    app_info.engine_version = Some(Version { major: 1, minor: 0, patch: 0 });

    let extensions = vg::get_required_instance_extensions(glfw).unwrap();

    Instance::new(Some(&app_info), &extensions, None).unwrap()
}

