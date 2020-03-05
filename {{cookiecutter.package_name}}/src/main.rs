use log::{debug, error, info, trace, warn};
use std::env;

fn main() -> Result<(), log4rs::Error> {
    let exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => panic!("failed to get current exe path: {}", e),
    };
    let exe_path = match exe_path.parent() {
        Some(exe_path) => exe_path,
        None => panic!("failed to get parent path"),
    };
    if cfg!(debug_assertions) {
        log4rs::init_file(
            exe_path.join("config/debug/log4rs.yaml"),
            Default::default(),
        )?;
    } else {
        log4rs::init_file(
            exe_path.join("config/release/log4rs.yaml"),
            Default::default(),
        )?;
    }

    error!("Goes to stderr");
    warn!("Goes to stderr");
    info!("Goes to stderr");
    debug!("Goes to stderr.");
    trace!("Goes to stderr.");
    info!(target:"app::requests", "Goes to requests");
    error!(target:"app::requests", "Goes to requests");

    Ok(())
}
