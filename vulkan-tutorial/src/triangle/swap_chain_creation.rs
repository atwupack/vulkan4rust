use glfw;
use glfw::{Glfw,Window,WindowMode,WindowHint,ClientApiHint};

use vulkano::instance;
use vulkano::instance::{Features, ApplicationInfo, Version, Instance, InstanceExtensions, PhysicalDevice, QueueFamily, DeviceExtensions};
use vulkano::instance::debug::{DebugCallback, Message};
use vulkano::device::{Device, Queue};
use vulkano::swapchain::Surface;

use vulkano_glfw as vg;

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
    //window: Window,
    _instance: Arc<Instance>,
    _callback: Option<DebugCallback>,
    _physical_device: usize,
    _device: Arc<Device>,
    _graphics_queue: Arc<Queue>,
    _present_queue: Arc<Queue>,
    surface: Arc<Surface<Window>>,
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
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let window = init_window(&mut glfw);

        // init vulkan instance
        let instance = create_instance(&glfw);
        let callback = setup_debug_callback(&instance);

        let surface = create_surface(&instance, window);

        let req_dev_exts = DeviceExtensions {
            khr_swapchain: true,
            .. DeviceExtensions::none()
        };

        let physical_device = pick_physical_device(&glfw, &instance, &req_dev_exts).unwrap();
        let (device, graphics_queue, present_queue) = create_logical_device(&glfw, physical_device, &req_dev_exts);

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

fn create_surface(instance: &Arc<Instance>, window: Window ) -> Arc<Surface<Window>> {
    vg::create_window_surface(instance.clone(), window).unwrap()
}

fn pick_physical_device<'a>(glfw: &Glfw, instance: &'a Arc<Instance>, req_exts: &DeviceExtensions) -> Option<PhysicalDevice<'a>> {
    for device in PhysicalDevice::enumerate(instance) {
        if is_device_suitable(glfw, device, req_exts) {
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

fn is_device_suitable<'a>(glfw: &Glfw, device: PhysicalDevice<'a>, req_exts: &DeviceExtensions) -> bool {
    let family = find_queue_families(glfw, device);
    family.is_some() && check_device_extension_support(device, req_exts)
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

fn debug_callback(msg: &Message) {
    println!("validation layer {}", msg.description)
}

fn init_window(glfw: &mut Glfw) -> Window {
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));
    let (window, _events) = glfw.create_window(WIDTH, HEIGHT, "Vulkan", WindowMode::Windowed).unwrap();
    window
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

