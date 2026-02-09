use anyhow::{Ok, Result};
use clap::Parser;

fn main() -> Result<()> {
    // Initialize logger, default info level, display file line number and time
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format(|buf, record| {
            use std::io::Write;
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{} {level_style}{}{level_style:#} {}:{}] {level_style}{}{level_style:#}",
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    let cli = vdisk::VDiskCli::try_parse()?;

    vdisk::run(cli).map_err(|e| {
        log::error!("Error: {e}");
        e
    })?;
    Ok(())
}
