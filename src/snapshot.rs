use std::process::Command;
use chrono::Local;
use colored::Colorize;

pub fn snapshot_make() {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let desc = format!("Pre fedora-update at {}", now);

    println!("{}", "Creating pre-update snapshot...".yellow());

    let status = Command::new("snapper")
        .args(["-c", "root", "create", "-t", "pre", "--description", &desc])
        .status();
    match status {
        Ok(s) if s.success() => println!("{}", format!("Snapshot created and saved as {}!",desc).yellow()),
        _ => eprintln!("{}", "Snapper failed (either not installed or not configured)! Proceeding anyway.".red())
    }
}