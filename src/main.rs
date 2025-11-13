mod hyprland_config;
mod shell_command;
mod system_info;
mod window_selector;

use shell_command::commands::options_exec;
use shell_command::types::*;

use clap::Parser;

fn main() {
    //window_selector::create_window();
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if cli.version {
        options_exec::version();
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Generate {
            float,
            persistentsize,
            tile,
            fullscreen,
        }) => {
            shell_command::commands::generate::exec(
                float.clone(),
                persistentsize.clone(),
                tile.clone(),
                fullscreen.clone(),
            );
        }
        None => {}
    }
}
