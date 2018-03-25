use glfw::{Glfw};

use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano::instance::debug::{DebugCallback};

use vulkano_glfw as vg;
use vulkano_glfw::GlfwWindow;

// import functions from previous parts
use ::triangle::setup::base_code::init_window;
use ::triangle::setup::validation_layers::create_instance;
use ::triangle::setup::validation_layers::setup_debug_callback;

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
    window: GlfwWindow,
    _instance: Arc<Instance>,
    _callback: Option<DebugCallback>,
    _physical_device: usize,
}

impl<'a> HelloTriangleApplication {
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

        let physical_device = pick_physical_device(&glfw, &instance).unwrap();

        HelloTriangleApplication {
            glfw: glfw,
            window: window,
            _instance: instance.clone(),
            _callback: callback,
            _physical_device: physical_device.index(),
        }
    }
}

fn pick_physical_device<'a>(glfw: &'a Glfw, instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
    for device in PhysicalDevice::enumerate(instance) {
        if is_device_suitable(glfw, &device) {
            println!("Using device: {}", device.name());
            return Some(device);
        }
    }
    None
}

fn is_device_suitable<'a>(glfw: &'a Glfw, device: &PhysicalDevice<'a>) -> bool {
    let family = find_queue_families(glfw, device);
    family.is_some()
}

fn find_queue_families<'a>(glfw: &'a Glfw, device: &PhysicalDevice<'a> ) -> Option<QueueFamily<'a>> {
    for family in device.queue_families() {
        if family.supports_graphics() && vg::get_physical_device_presentation_support(glfw, &family) {
            return Some(family);
        }
    }
    None
}

