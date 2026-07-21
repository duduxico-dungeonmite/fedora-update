use std::process::Command;
use chrono::Local;

pub fn snapshot_make() {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let desc = format!("Pre fedora-update at {}", now);

    println!("Creating pre-update snapshot...");

    let status = Command::new("sudo")
        .args(["snapper", "-c", "root", "create", "-t", "pre", "--description", &desc])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapshot created and saved as {desc}!"),
        _ => eprintln!("Snapper failed (either not installed or not configured)! Proceeding anyway.")
    }
}