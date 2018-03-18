use glfw::{Glfw,Window};

use vulkano::instance;
use vulkano::instance::{ApplicationInfo, Version, Instance, InstanceExtensions};
use vulkano::instance::debug::{DebugCallback, Message};

use vulkano_glfw as vg;

// import functions from previous parts
use ::triangle::setup::base_code::init_window;

use std::borrow::Cow;
use std::sync::Arc;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: &[&str; 1] = &["VK_LAYER_LUNARG_standard_validation"];

const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

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
    _callback: Option<DebugCallback>,
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

        let (glfw, window) = init_window(WIDTH, HEIGHT);

        // init vulkan instance
        let instance = create_instance(&glfw);
        let callback = setup_debug_callback(&instance);

        HelloTriangleApplication {
            glfw: glfw,
            window: window,
            _instance: instance,
            _callback: callback,
        }
    }
}

fn debug_callback(msg: &Message) {
    println!("validation layer {}", msg.description)
}

pub fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
    if ENABLE_VALIDATION_LAYERS {
        DebugCallback::errors_and_warnings(instance,debug_callback).ok()
    }
    else {
        None
    }
}

pub fn create_instance(glfw: &Glfw) -> Arc<Instance> {
    if ENABLE_VALIDATION_LAYERS && !check_validation_layer_support() {
        panic!("validation layers requested, but not available!");
    }

    // initVulkan stuff is here
    let mut app_info = ApplicationInfo::default();
    app_info.application_name = Some(Cow::Borrowed("Hello Triangle"));
    app_info.application_version = Some(Version { major: 1, minor: 0, patch: 0 });
    app_info.engine_name = Some(Cow::Borrowed("No Engine"));
    app_info.engine_version = Some(Version { major: 1, minor: 0, patch: 0 });

    let extensions = InstanceExtensions {
        ext_debug_report: true,
        .. vg::get_required_instance_extensions(glfw).unwrap()
    };

    Instance::new(Some(&app_info), &extensions, VALIDATION_LAYERS).unwrap()
}

fn check_validation_layer_support() -> bool {
    for layer_name in VALIDATION_LAYERS {
        let mut layer_found = false;
        for layer in instance::layers_list().unwrap() {
            if layer.name() == *layer_name {
                layer_found = true;
            }
        }
        if !layer_found {
            return false;
        }
    }
    true
}

