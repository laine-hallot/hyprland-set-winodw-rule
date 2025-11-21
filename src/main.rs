mod hyprland_config;
mod shell_command;
mod system_info;
mod tui;
mod wayland;

use std::time::Duration;

use shell_command::commands::options_exec;
use shell_command::types::*;

use color_eyre::{Result, eyre};
use eyre::Error;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};

use clap::Parser;

fn main() -> Result<()> {
    system_info::get_data_dir();
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
    let cmd_result = match &cli.command {
        Some(Commands::Generate {
            float,
            persistentsize,
            tile,
            fullscreen,
        }) => {
            return shell_command::commands::generate::exec(
                float.clone(),
                persistentsize.clone(),
                tile.clone(),
                fullscreen.clone(),
            );
        }
        None => Err(Error::msg("Unknown option")),
    };

    // I think the tui library should handle displaying the error result so just return the raw result for now
    return cmd_result;
}
