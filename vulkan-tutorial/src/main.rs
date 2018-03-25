extern crate glfw;

extern crate clap;
use clap::*;

#[macro_use]
extern crate vulkano_shader_derive;

extern crate vulkano;

extern crate vk_sys;
extern crate vulkano_glfw;

mod triangle;
mod util;

use triangle::hello_triangle;

struct Command<'a> {
    name: &'a str,
    description: &'a str,
    main_function: fn(),
}

const TUTORIALS: &[Command; 1] = &[
    Command {
        name: "hello_triangle",
        description: "Hello triangle",
        main_function: hello_triangle::app_main,
    },
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
