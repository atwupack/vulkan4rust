use glfw;
use glfw::{Glfw,Window,WindowMode,WindowHint,ClientApiHint};

use vulkano::instance::{ApplicationInfo, Version, Instance};

use vulkano_glfw as vg;

use std::borrow::Cow;
use std::sync::Arc;

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
    window: Window,
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
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let window = init_window(&mut glfw);

        // init Vulkan
        let instance = create_instance(&glfw);

        HelloTriangleApplication {
            glfw: glfw,
            window: window,
            _instance: instance,
        }
    }
}

fn init_window(glfw: &mut Glfw) -> Window {
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));
    let (window, _events) = glfw.create_window(WIDTH, HEIGHT, "Vulkan", WindowMode::Windowed).unwrap();
    window
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

