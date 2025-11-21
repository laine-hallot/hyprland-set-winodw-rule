use color_eyre::Result;

use crate::tui::root::tui_root;

pub fn exec(float: bool, _persistentsize: bool, tile: bool, _fullscreen: bool) -> Result<()> {
    let selected_client = tui_root()?;
    match selected_client {
        Some(client) => {
            println!("Selected: {}", client.title);
            println!("Config: ");
            if float {
                println!("windowrule = float, initialTitle:{}", client.initial_title);
            }
            if tile {
                println!("windowrule = tile, initialTitle:{}", client.initial_title);
            }
        }
        _ => (),
    };

    Ok(())
}
