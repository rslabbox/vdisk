mod cli;
pub mod commands;
pub mod fs;
pub mod gpt;
mod io;
pub mod types;
mod utils;
pub mod fatfs;

pub use cli::VDiskCli;
pub use commands::run;

#[macro_use]
extern crate log;