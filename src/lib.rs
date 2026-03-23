mod cli;
pub mod commands;
pub mod fatfs;
pub mod fs;
pub mod gpt;
mod io;
pub mod types;
mod utils;

pub use cli::VDiskCli;
pub use commands::run;
pub use commands::{cat, cp, info, ls, mkdir, mkgpt, mkfs, mkimg, mv, rm};

#[macro_use]
extern crate log;
