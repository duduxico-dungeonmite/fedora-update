use crate::command::run_command;

pub fn cleanup() {
    run_command("Cleaning DNF cache", "dnf",&["clean","all"]);
    run_command("Removing unused Flatpak files...", "flatpak",&["uninstall","--unused"])
}
