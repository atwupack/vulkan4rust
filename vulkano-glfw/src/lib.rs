extern crate vulkano;
extern crate vk_sys;
extern crate glfw;

use std::sync::Arc;
use std::ptr;
use std::error;
use std::fmt;
use std::ffi::CString;

use std::sync::mpsc::Receiver;

use vulkano::VulkanObject;
use vulkano::instance::{Instance, InstanceExtensions, RawInstanceExtensions, QueueFamily};

use vulkano::swapchain::{Surface};

use glfw::{Window, Context, Glfw, WindowMode, WindowEvent};

pub struct GlfwWindow {
    window: Window,
}

impl From<Window> for GlfwWindow {
    fn from(window: Window) -> Self {
        GlfwWindow {
            window: window,
        }
    }
}

pub fn create_glfw_window(glfw: Glfw, width: u32, height: u32, title: &str, mode: WindowMode) -> Option<(GlfwWindow, Receiver<(f64, WindowEvent)>)> {
    match glfw.create_window(width, height, title, mode) {
        Some((window, events)) => Some((GlfwWindow::from(window), events)),
        None => None,
    }
}

impl GlfwWindow {
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}

unsafe impl Send for GlfwWindow {}
unsafe impl Sync for GlfwWindow {}

/// error while creating a GLFW-based surface
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VulkanoGlfwError {
    /// General GLFW error
    GlfwError{ code: u32 },
    NoExtensions,
}

impl error::Error for VulkanoGlfwError {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            VulkanoGlfwError::GlfwError{..} => "Genral Vulkan GLFW error",
            VulkanoGlfwError::NoExtensions => "Could not load required extensions",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}

impl fmt::Display for VulkanoGlfwError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", error::Error::description(self))
    }
}

/// Create a surface from a GLFW window
pub fn create_window_surface(instance: Arc<Instance>, window: GlfwWindow ) -> Result<Arc<Surface<GlfwWindow>>, VulkanoGlfwError> {
    let internal_instance = instance.as_ref().internal_object();
    let internal_window = window.window.window_ptr();
    let mut internal_surface: vk_sys::SurfaceKHR = 0;
    let result = unsafe {
        glfw::ffi::glfwCreateWindowSurface(internal_instance, internal_window, ptr::null(), &mut internal_surface as *mut u64 )
    };
    if result != vk_sys::SUCCESS {
        return Err(VulkanoGlfwError::GlfwError { code: result });
    }
    Ok(Arc::new(unsafe {
        Surface::from_raw_surface(instance, internal_surface, window)
    }))
}

/// create InstanceExtensions from required GLFW extensions
pub fn get_required_instance_extensions(glfw: &Glfw) -> Result<InstanceExtensions, VulkanoGlfwError> {
    get_required_raw_instance_extensions(glfw).and_then(|rie| {
        Ok(InstanceExtensions::from(&rie))
    })
}

/// create RawInstanceExtensions from required GLFW extensions
pub fn get_required_raw_instance_extensions(glfw: &Glfw) -> Result<RawInstanceExtensions, VulkanoGlfwError> {
    let exts = glfw.get_required_instance_extensions();
    if exts.is_none() {
        return Err(VulkanoGlfwError::NoExtensions);
    }

    let iter = exts.unwrap().into_iter().map(|s| {
        let new_c_string = CString::new(s);
        new_c_string.unwrap()
    });

    Ok(RawInstanceExtensions::new(iter))
}

/// This function returns whether the specified queue family of the specified physical device supports presentation to the platform GLFW was built for.
pub fn get_physical_device_presentation_support(glfw: &Glfw, family: &QueueFamily) -> bool {
    let device = family.physical_device();
    let internal_device = device.internal_object();
    let instance = device.instance();
    let internal_instance = instance.as_ref().internal_object();
    glfw.get_physical_device_presentation_support_raw(internal_instance, internal_device, family.id())
}

