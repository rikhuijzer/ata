use crate::config::ConfigLocation;

use clap::Parser;

/// Ask the Terminal Anything (ATA): OpenAI GPT in the terminal
#[derive(Parser, Debug)]
#[command(author = crate_authors!("\n\t"), version = crate_version!(),
    about, long_about = None,
    help_template = "{before-help}{name} {version} — {about}\
    \n\n\
    © 2023\t{author}\
    \n\n\
    {usage-heading} {usage}\
    \n\n\
    {all-args}{after-help}")]
pub struct Ata {
    /// Path to the configuration TOML file.
    #[arg(short = 'c', long = "config", default_value = "")]
    pub config: ConfigLocation,

    /// Avoid printing the configuration to stdout.
    #[arg(long)]
    pub hide_config: bool,

    /// Print the keyboard shortcuts.
    #[arg(long)]
    pub print_shortcuts: bool,
}
