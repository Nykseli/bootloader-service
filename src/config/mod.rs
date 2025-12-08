use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ConfigArgs {
    /// Use session/user message bus connection instead of system
    #[arg(short, long, default_value_t = false)]
    pub session: bool,
}

#[cfg(not(feature = "dev"))]
pub const GRUB_FILE_PATH: &'static str = "/etc/default/grub";
#[cfg(feature = "dev")]
pub const GRUB_FILE_PATH: &'static str = "tmp/grub";

#[cfg(not(feature = "dev"))]
pub const GRUB_ROOT_PATH: &'static str = "/etc/default";
#[cfg(feature = "dev")]
pub const GRUB_ROOT_PATH: &'static str = "tmp";
