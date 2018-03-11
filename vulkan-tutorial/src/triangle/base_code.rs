use glfw;
use glfw::{Glfw,Window,WindowMode,WindowHint,ClientApiHint};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn app_main() {
    let mut app = HelloTriangleApplication::new();

    app.run();
}

struct HelloTriangleApplication {
    glfw: Glfw,
    window: Window,
}

impl HelloTriangleApplication {
    fn run(&mut self) {
        self.main_loop();
        self.cleanup();
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

        HelloTriangleApplication {
            glfw: glfw,
            window: window,
        }
    }
}

fn init_window(glfw: &mut Glfw) -> Window {
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));
    let (window, _events) = glfw.create_window(WIDTH, HEIGHT, "Vulkan", WindowMode::Windowed).unwrap();
    window
}


