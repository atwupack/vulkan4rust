use glfw::{Glfw};

use vulkano::instance::{Features, Instance, PhysicalDevice, QueueFamily, DeviceExtensions};
use vulkano::instance::debug::{DebugCallback};
use vulkano::device::{Device, Queue};
use vulkano::swapchain::Surface;

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
    //window: Window,
    _instance: Arc<Instance>,
    _callback: Option<DebugCallback>,
    _physical_device: usize,
    _device: Arc<Device>,
    _graphics_queue: Arc<Queue>,
    _present_queue: Arc<Queue>,
    surface: Arc<Surface<GlfwWindow>>,
}

impl<'a> HelloTriangleApplication {
    fn run(&mut self) -> Result<(),()> {
        self.main_loop();
        self.cleanup();
        Ok(())
    }

    fn main_loop(&mut self) {
        while !self.surface.window().should_close() {
            self.glfw.poll_events();
        }
    }

    fn cleanup(&mut self) {
        // destroy window and terminate is not needed
        // because it is handled by the library
    }

    fn new() -> HelloTriangleApplication {

        let (glfw, window) = init_window(WIDTH, HEIGHT);

        // init vulkan instance
        let instance = create_instance(&glfw);
        let callback = setup_debug_callback(&instance);

        let surface = create_surface(&instance, window);

        let physical_device = pick_physical_device(&glfw, &instance).unwrap();
        let (device, graphics_queue, present_queue) = create_logical_device(&glfw, physical_device);

        HelloTriangleApplication {
            glfw: glfw,
            //window: window,
            _instance: instance.clone(),
            _callback: callback,
            _physical_device: physical_device.index(),
            _device: device,
            _graphics_queue: graphics_queue,
            _present_queue: present_queue,
            surface: surface,
        }
    }
}

pub fn create_surface(instance: &Arc<Instance>, window: GlfwWindow ) -> Arc<Surface<GlfwWindow>> {
    vg::create_window_surface(instance.clone(), window).unwrap()
}

fn pick_physical_device<'a>(glfw: &Glfw, instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
    for device in PhysicalDevice::enumerate(instance) {
        if is_device_suitable(glfw, &device) {
            println!("Using device: {}", device.name());
            return Some(device);
        }
    }
    None
}

fn create_logical_device<'a>(glfw: &Glfw, phys: PhysicalDevice<'a>) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {
    let family = find_queue_families(glfw, &phys).unwrap();
    let (device, mut qiter) = Device::new(phys, &Features::none(),
                                &DeviceExtensions::none(),
                                vec![(family, 1.0)]).unwrap();
    let queue = qiter.next().unwrap();
    (device, queue.clone(), queue.clone())
}

fn is_device_suitable<'a>(glfw: &Glfw, device: &PhysicalDevice<'a>) -> bool {
    let family = find_queue_families(glfw, device);
    family.is_some()
}

fn find_queue_families<'a>(glfw: &Glfw, device: &PhysicalDevice<'a> ) -> Option<QueueFamily<'a>> {
    for family in device.queue_families() {
        if family.supports_graphics() && vg::get_physical_device_presentation_support(glfw, &family)  {
            return Some(family);
        }
    };

    None
}

