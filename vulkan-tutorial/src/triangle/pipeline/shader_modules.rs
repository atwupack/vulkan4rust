use glfw::{Glfw,Window};

use vulkano::instance::{Features, Instance, PhysicalDevice, QueueFamily, DeviceExtensions};
use vulkano::instance::debug::{DebugCallback};
use vulkano::device::{Device, Queue};
use vulkano::swapchain::{Surface, Capabilities, SupportedPresentModes, ColorSpace, PresentMode, Swapchain, CompositeAlpha};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::sync::SharingMode;

use vulkano_glfw as vg;

// import functions from previous parts
use ::triangle::setup::base_code::init_window;
use ::triangle::setup::validation_layers::create_instance;
use ::triangle::setup::validation_layers::setup_debug_callback;

use std::sync::Arc;
use std::cmp::{min, max};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450
#extension GL_ARB_separate_shader_objects : enable

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) out vec3 fragColor;

vec2 positions[3] = vec2[](
    vec2(0.0, -0.5),
    vec2(0.5, 0.5),
    vec2(-0.5, 0.5)
);

vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    fragColor = colors[gl_VertexIndex];
}
"]
    struct Dummy;
}

mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(fragColor, 1.0);
}
"]
    struct Dummy;
}

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
    surface: Arc<Surface<Window>>,
    _swap_chain: Arc<Swapchain<Window>>,
    _swap_chain_images: Vec<Arc<SwapchainImage<Window>>>,
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

        let req_dev_exts = DeviceExtensions {
            khr_swapchain: true,
            .. DeviceExtensions::none()
        };

        let physical_device = pick_physical_device(&glfw, &instance, &req_dev_exts, &surface).unwrap();
        let (device, graphics_queue, present_queue) = create_logical_device(&glfw, physical_device, &req_dev_exts);

        let (swap_chain, images) = create_swap_chain(&device, &surface, &graphics_queue);

        create_image_views();
        create_graphics_pipeline(&device);

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
            _swap_chain: swap_chain,
            _swap_chain_images: images,
        }
    }
}

fn create_graphics_pipeline(device: &Arc<Device>) {
    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");
}

fn create_image_views() {
    // it seems this is not needed with vulkano
}

fn query_swap_chain_support(surface: &Arc<Surface<Window>>, device: PhysicalDevice) -> Capabilities {
    surface.capabilities(device).unwrap()
}

fn create_swap_chain(device: &Arc<Device>, surface: &Arc<Surface<Window>>, queue: &Arc<Queue>) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let caps = query_swap_chain_support(&surface, device.physical_device());

    let req_image_count = caps.min_image_count + 1;
    let image_count = match caps.max_image_count {
        Some(max_image) => if req_image_count > max_image {
            max_image
        }
        else {
            req_image_count
        }
        None => req_image_count,
    };

    let (format, _color_space) = choose_swap_surface_format(&caps);
    let extend = choose_swap_extend(&caps);

    Swapchain::new(device.clone(),
                        surface.clone(),
                        image_count,
                        format,
                        extend,
                        1, // layers
                        ImageUsage {
                            color_attachment: true,
                            .. ImageUsage::none()
                        },
                        SharingMode::from(queue),
                        caps.current_transform,
                        CompositeAlpha::Opaque,
                        choose_swap_present_mode(&caps),
                        true, // clipped
                        None // old swapchain
                        ).unwrap()
}

fn choose_swap_surface_format(caps: &Capabilities) -> (Format, ColorSpace) {
    let avail_formats = &caps.supported_formats;
    if avail_formats.len() == 0 {
        (Format::B8G8R8Unorm, ColorSpace::SrgbNonLinear)
    }
    else {
        if avail_formats.contains(&(Format::B8G8R8Unorm, ColorSpace::SrgbNonLinear)) {
            (Format::B8G8R8Unorm, ColorSpace::SrgbNonLinear)
        }
        else {
            avail_formats[0]
        }
    }
}

fn choose_swap_present_mode(caps: &Capabilities) -> PresentMode {
    let avail_modes = caps.present_modes;
    if avail_modes.mailbox {
        PresentMode::Mailbox
    }
    else {
        if avail_modes.immediate {
            PresentMode::Immediate
        }
        else {
            PresentMode::Fifo
        }
    }
}

fn choose_swap_extend(caps: &Capabilities) -> [u32;2] {
    match caps.current_extent {
        Some(e) => e,
        None => {
            let width = max(caps.min_image_extent[0], min(caps.max_image_extent[0], WIDTH));
            let height = max(caps.min_image_extent[1], min(caps.max_image_extent[1], HEIGHT));
            [width, height]
        }
    }
}

fn create_surface(instance: &Arc<Instance>, window: Window ) -> Arc<Surface<Window>> {
    vg::create_window_surface(instance.clone(), window).unwrap()
}

fn pick_physical_device<'a>(glfw: &Glfw, instance: &'a Arc<Instance>, req_exts: &DeviceExtensions, surface: &Arc<Surface<Window>>) -> Option<PhysicalDevice<'a>> {
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

fn is_device_suitable<'a>(glfw: &Glfw, device: PhysicalDevice<'a>, req_exts: &DeviceExtensions, surface: &Arc<Surface<Window>>) -> bool {
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

