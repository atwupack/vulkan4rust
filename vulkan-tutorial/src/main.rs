extern crate glfw;

extern crate clap;
use clap::*;

#[macro_use]
extern crate vulkano_shader_derive;

#[macro_use]
extern crate vulkano;

extern crate vk_sys;
extern crate vulkano_glfw;

mod triangle;
mod util;

use triangle::setup::base_code;
use triangle::setup::instance_creation;
use triangle::setup::validation_layers;
use triangle::setup::physical_device_selection;
use triangle::setup::logical_device;
use triangle::presentation::window_surface;
use triangle::presentation::swap_chain_creation;
use triangle::presentation::image_views;
use triangle::pipeline::graphics_pipeline;
use triangle::pipeline::shader_modules;
use triangle::pipeline::fixed_functions;
use triangle::pipeline::render_passes;
use triangle::pipeline::graphics_pipeline_complete;
use triangle::drawing::framebuffers;


struct Command<'a> {
    name: &'a str,
    description: &'a str,
    main_function: fn(),
}

const TUTORIALS: &[Command; 14] = &[
    Command {
        name: "00_base_code",
        description: "Base code",
        main_function: base_code::app_main,
    },
    Command {
        name: "01_instance_creation",
        description: "Instance",
        main_function: instance_creation::app_main,
    },
    Command {
        name: "02_validation_layers",
        description: "Validation layers",
        main_function: validation_layers::app_main,
    },
    Command {
        name: "03_physical_device_selection",
        description: "Physical devices and queue families",
        main_function: physical_device_selection::app_main,
    },
    Command {
        name: "04_logical_device",
        description: "Logical device and queues",
        main_function: logical_device::app_main,
    },
    Command {
        name: "05_window_surface",
        description: "Window surface",
        main_function: window_surface::app_main,
    },
    Command {
        name: "06_swap_chain_creation",
        description: "Swap chain",
        main_function: swap_chain_creation::app_main,
    },
    Command {
        name: "07_image_views",
        description: "Image views",
        main_function: image_views::app_main,
    },
    Command {
        name: "08_graphics_pipeline",
        description: "Graphics pipeline",
        main_function: graphics_pipeline::app_main,
    },
    Command {
        name: "09_shader_modules",
        description: "Shader modules",
        main_function: shader_modules::app_main,
    },
    Command {
        name: "10_fixed_functions",
        description: "Fixed functions",
        main_function: fixed_functions::app_main,
    },
    Command {
        name: "11_render_passes",
        description: "Render passes",
        main_function: render_passes::app_main,
    },
    Command {
        name: "12_graphics_pipeline_complete",
        description: "Graphics pipeline complete",
        main_function: graphics_pipeline_complete::app_main,
    },
    Command {
        name: "13_framebuffers",
        description: "Framebuffers",
        main_function: framebuffers::app_main,
    }
];

const UTILS: &[Command; 1] = &[
    Command {
        name: "devices",
        description: "List physical devices",
        main_function: util::info::show_physical_devices,
    },
];


fn find_command<'a>(commands: &'a [Command], name: &str) -> Option<&'a Command<'a>> {
    for t in commands {
        if t.name == name {
            return Some(t);
        }
    }
    None
}

fn add_sub_command<'a>(app: App<'static, 'static>, name: &'a str, about: &'static str, commands: &'static [Command]) -> App<'static, 'static> {
    let mut sub_command = SubCommand::with_name(name).about(about);
    for t in commands {
        sub_command = sub_command.subcommand(SubCommand::with_name(t.name).about(t.description));
    }
    app.subcommand(sub_command)
}

fn execute_command(name: &str, matches: &ArgMatches) {
    let command_matches = matches.subcommand_matches(name).unwrap();
    let sub_name = command_matches.subcommand_name().unwrap();
    let command = if name=="run" {
        find_command(TUTORIALS , sub_name)
    }
    else if name=="show" {
        find_command(UTILS , sub_name)
    }
    else {
        panic!("Unknown subcommand")
    };

    match command {
        Some(t) => (t.main_function)(),
        None => panic!("Unknown executable")
    }

}

fn main() {
    let mut app = App::new("Vulkan Tutorial")
                    .version("1.0")
                    .author("André Twupack <atwupack@mailbox.org>")
                    .about("Vulkan Tutorials from vulkan-tutorial.com");

    app = add_sub_command(app, "run", "Run a tutorial", TUTORIALS);
    app = add_sub_command(app, "show", "Show system info", UTILS);

    let matches = app.get_matches();

    let sub = matches.subcommand_name();
    match sub {
        Some(name) => {
            execute_command(name, &matches);
        },
        None => panic!("Unknown subcommand")
    }
}
