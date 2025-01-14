use std::fs;

mod controller;
pub mod controller_type;
mod cpu;
mod cpuset;
mod dbus;
pub mod manager;
mod pids;

/// Checks if the system was booted with systemd
pub fn booted() -> bool {
    fs::symlink_metadata("/run/systemd/system")
        .map(|p| p.is_dir())
        .unwrap_or_default()
}
