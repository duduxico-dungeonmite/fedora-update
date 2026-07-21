use crate::command::run_command;

pub fn update_dnf() {
   run_command("Updating DNF...", "dnf",&["update", "-y"])
}
