use glfw::{Glfw,Window};

use vulkano::instance;
use vulkano::instance::{ApplicationInfo, Version, Instance, InstanceExtensions, PhysicalDevice, QueueFamily};
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

fn debug_callback(msg: &Message) {
    println!("validation layer {}", msg.description)
}

fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
    if ENABLE_VALIDATION_LAYERS {
        DebugCallback::errors_and_warnings(instance,debug_callback).ok()
    }
    else {
        None
    }
}

fn create_instance(glfw: &Glfw) -> Arc<Instance> {
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

