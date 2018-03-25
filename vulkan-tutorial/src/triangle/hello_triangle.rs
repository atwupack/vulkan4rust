use glfw;
use glfw::{Glfw,WindowMode,WindowHint,ClientApiHint};

use vulkano::instance;
use vulkano::instance::{InstanceExtensions, ApplicationInfo, Version, Instance, Features, PhysicalDevice, QueueFamily, DeviceExtensions};
use vulkano::instance::debug::{DebugCallback, Message};
use vulkano::device::{Device, Queue};
use vulkano::swapchain;
use vulkano::swapchain::{Surface, Capabilities, SupportedPresentModes, ColorSpace, PresentMode, Swapchain, CompositeAlpha};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage, ImageLayout};
use vulkano::sync::SharingMode;
use vulkano::framebuffer::{ Subpass, Framebuffer, RenderPass,RenderPassDescClearValues, LoadOp, StoreOp, RenderPassDesc, LayoutAttachmentDescription, LayoutPassDescription, LayoutPassDependencyDescription};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::BufferlessVertices;
use vulkano::pipeline::viewport::{Viewport, Scissor};
use vulkano::format::ClearValue;

use vulkano_glfw as vg;
use vulkano_glfw::{create_glfw_window, GlfwWindow};

use std::sync::Arc;
use std::cmp::{max, min};
use std::borrow::Cow;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: &[&str; 1] = &["VK_LAYER_LUNARG_standard_validation"];
const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

pub mod vs {
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

pub mod fs {
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

fn create_graphics_pipeline(device: &Arc<Device>, swapchain: &Arc<Swapchain<GlfwWindow>>,
        render_pass: &Arc<RenderPass<CustomRenderPassDesc>>, images: Vec<Arc<SwapchainImage<GlfwWindow>>>, queue: &Arc<Queue>) {
    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [swapchain.dimensions()[0] as f32, swapchain.dimensions()[1] as f32],
        depth_range: 0.0 .. 1.0,
    };

    let scissor = Scissor {
        origin: [0,0],
        dimensions: swapchain.dimensions(),
    };

    let pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_shader(vs.main_entry_point(), ())
        .triangle_list()
        .viewports_scissors(Some((viewport, scissor)))
        //.viewports_dynamic_scissors_irrelevant(1)
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap());

    let mut framebuffers = Vec::new();

    for image in images {
        framebuffers.push(Arc::new(Framebuffer::start(render_pass.clone()).add(image).unwrap().build().unwrap()));
    }

    for fb in framebuffers {
        let _cb = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
            .begin_render_pass(fb.clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()]).unwrap()
            .draw(pipeline.clone(),
                DynamicState::none(),
                BufferlessVertices {
                    vertices: 3,
                    instances: 1,
                }, (),()).unwrap()
            .end_render_pass().unwrap()
            .build().unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct CustomRenderPassDesc {
    swapchain: Arc<Swapchain<GlfwWindow>>,
}

unsafe impl RenderPassDescClearValues<Vec<ClearValue>> for CustomRenderPassDesc {
    fn convert_clear_values(&self, values: Vec<ClearValue>) -> Box<Iterator<Item = ClearValue>> {
        Box::new(values.into_iter())
    }
}

unsafe impl RenderPassDesc for CustomRenderPassDesc {
    fn num_attachments(&self) -> usize {
        1
    }

    fn attachment_desc(&self, _num: usize) -> Option<LayoutAttachmentDescription> {
        Some(LayoutAttachmentDescription {
            format: self.swapchain.format(),
            samples: 1,
            load: LoadOp::Clear,
            store: StoreOp::Store,
            stencil_load: LoadOp::DontCare,
            stencil_store: StoreOp::DontCare,
            initial_layout: ImageLayout::Undefined,
            final_layout: ImageLayout::PresentSrc,
        })
    }

    fn num_subpasses(&self) -> usize {
        1
    }

    fn subpass_desc(&self, _num: usize) -> Option<LayoutPassDescription> {
        Some(LayoutPassDescription {
            color_attachments: vec![(0, ImageLayout::ColorAttachmentOptimal)],
            depth_stencil: None,
            input_attachments: vec![],
            resolve_attachments: vec![],
            preserve_attachments: vec![],
        })
    }

    fn num_dependencies(&self) -> usize {
        0
    }

    fn dependency_desc(&self, _num: usize) -> Option<LayoutPassDependencyDescription> {
        None
    }
}

fn create_render_pass(device: &Arc<Device>, swapchain: &Arc<Swapchain<GlfwWindow>>) -> Arc<RenderPass<CustomRenderPassDesc>> {
    let rpd = CustomRenderPassDesc {
        swapchain: swapchain.clone(),
    };
    Arc::new(rpd.build_render_pass(device.clone()).unwrap())
}

fn query_swap_chain_support(surface: &Arc<Surface<GlfwWindow>>, device: PhysicalDevice) -> Capabilities {
    surface.capabilities(device).unwrap()
}

fn create_swap_chain(device: &Arc<Device>, surface: &Arc<Surface<GlfwWindow>>, queue: &Arc<Queue>) -> (Arc<Swapchain<GlfwWindow>>, Vec<Arc<SwapchainImage<GlfwWindow>>>) {
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


fn create_surface(instance: &Arc<Instance>, window: GlfwWindow ) -> Arc<Surface<GlfwWindow>> {
    vg::create_window_surface(instance.clone(), window).unwrap()
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

fn init_window(width: u32, height: u32) -> (Glfw, GlfwWindow) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));
    let (window, _events) = create_glfw_window(glfw, width, height, "Vulkan", WindowMode::Windowed).unwrap();
    (glfw,window)
}
