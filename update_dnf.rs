use crate::command::run_command;

pub fn update_dnf() {
   run_command("Updating DNF...", "sudo",&["dnf5","update", "-y"])
}
