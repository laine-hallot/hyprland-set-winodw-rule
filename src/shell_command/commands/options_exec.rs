use crate::system_info::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui template with crossterm and tokio")]
struct Args {
    /// App tick rate
    #[arg(short, long, default_value_t = 1000)]
    app_tick_rate: u64,
}

pub fn version() -> String {
    let author = clap::crate_authors!();

    let commit_hash = env!("RATATUI_TEMPLATE_GIT_INFO");

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().unwrap().display().to_string();
    let data_dir_path = get_data_dir().unwrap().display().to_string();

    format!(
        "\
{commit_hash}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}
