use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Display version info
    #[arg(short, long)]
    pub version: bool,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Generate {
        #[arg(long, help = "add float rule")]
        float: bool,

        #[arg(long, help = "add persistentsize rule")]
        persistentsize: bool,

        #[arg(long, help = "add tile rule")]
        tile: bool,

        #[arg(long, help = "add fullscreen rule")]
        fullscreen: bool,
    },
}
