use crate::command::run_command;

pub fn update_flatpak() {
    run_command("Updating Flatpak...", "flatpak", &["upgrade", "-y"]);
}