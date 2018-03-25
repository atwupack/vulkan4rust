use glfw::{Glfw};

use vulkano::instance::{Features, Instance, PhysicalDevice, QueueFamily, DeviceExtensions};
use vulkano::instance::debug::{DebugCallback};
use vulkano::device::{Device, Queue};
use vulkano::swapchain;
use vulkano::swapchain::{Surface, SupportedPresentModes, Swapchain};

use vulkano_glfw as vg;
use vulkano_glfw::GlfwWindow;

// import functions from previous parts
use ::triangle::setup::base_code::init_window;
use ::triangle::setup::validation_layers::create_instance;
use ::triangle::setup::validation_layers::setup_debug_callback;
use ::triangle::presentation::window_surface::create_surface;
use ::triangle::presentation::swap_chain_creation::create_swap_chain;
use ::triangle::presentation::swap_chain_creation::query_swap_chain_support;
use ::triangle::drawing::command_buffers::create_graphics_pipeline;
use ::triangle::pipeline::render_passes::{create_render_pass};

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
    swap_chain: Arc<Swapchain<GlfwWindow>>,
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
            self.draw_frame();
        }
    }

    fn draw_frame(&self) {
        let (image_num, acquire_future) = swapchain::acquire_next_image(self.swap_chain.clone(), None).unwrap();
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

        let req_dev_exts = DeviceExtensions {
            khr_swapchain: true,
            .. DeviceExtensions::none()
        };

        let physical_device = pick_physical_device(&glfw, &instance, &req_dev_exts, &surface).unwrap();
        let (device, graphics_queue, present_queue) = create_logical_device(&glfw, physical_device, &req_dev_exts);

        let (swapchain, images) = create_swap_chain(&device, &surface, &graphics_queue);

        create_image_views();
        let render_pass = create_render_pass(&device, &swapchain);
        create_graphics_pipeline(&device, &swapchain, &render_pass, images, &graphics_queue);

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
            swap_chain: swapchain,
        }
    }
}

fn create_image_views() {
    // it seems this is not needed with vulkano
}

fn pick_physical_device<'a>(glfw: &Glfw, instance: &'a Arc<Instance>, req_exts: &DeviceExtensions, surface: &Arc<Surface<GlfwWindow>>) -> Option<PhysicalDevice<'a>> {
    for device in PhysicalDevice::enumerate(instance) {
        if is_device_suitable(glfw, device, req_exts, surface) {
            println!("Using device: {}", device.name());
            return Some(device);
        }
    }
    None
}

fn create_logical_device<'a>(glfw: &Glfw, phys: PhysicalDevice<'a>, req_exts: &DeviceExtensions) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {
    let family = find_queue_families(glfw, phys).unwrap();
    let (device, mut qiter) = Device::new(phys, &Features::none(),
                                req_exts,
                                vec![(family, 1.0)]).unwrap();
    let queue = qiter.next().unwrap();
    (device, queue.clone(), queue.clone())
}

fn is_device_suitable<'a>(glfw: &Glfw, device: PhysicalDevice<'a>, req_exts: &DeviceExtensions, surface: &Arc<Surface<GlfwWindow>>) -> bool {
    let family = find_queue_families(glfw, device);
    let caps = query_swap_chain_support(surface, device);
    family.is_some() && surface.is_supported(family.unwrap()).unwrap() && check_device_extension_support(device, req_exts)
        && !caps.supported_formats.is_empty() && caps.present_modes != SupportedPresentModes::none()
}

fn check_device_extension_support(device: PhysicalDevice, req_exts: &DeviceExtensions) -> bool {
    let supported_ext = DeviceExtensions::supported_by_device(device);
    req_exts.intersection(&supported_ext) == *req_exts
}

fn find_queue_families<'a>(glfw: &Glfw, device: PhysicalDevice<'a> ) -> Option<QueueFamily<'a>> {
    for family in device.queue_families() {
        if family.supports_graphics() && vg::get_physical_device_presentation_support(glfw, &family)  {
            return Some(family);
        }
    };

    None
}

