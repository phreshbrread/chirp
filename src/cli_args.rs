use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    //#[arg(short, long)]
    //system: Option<String>,
    /// Emulate original hardware behaviour
    #[arg(short, long)]
    pub original_behaviour: bool,

    /// Display current framerate
    #[arg(short = 'f', long)]
    pub show_fps: bool,

    /// Enable using the ESC key to exit
    #[arg(short, long)]
    pub esc_quits: bool,

    /// Path to a CHIP-8 ROM file
    pub rom_path: PathBuf,
}

pub fn parse_cli_args() -> Args {
    return Args::parse();
}
