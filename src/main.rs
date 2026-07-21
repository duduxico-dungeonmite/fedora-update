use crate::cache_clean::cleanup;
use crate::quotes::end_quote;
use crate::reboot::reboot_check;
use crate::snapshot::snapshot_make;
use crate::update_dnf::update_dnf;
use crate::update_flatpak::update_flatpak;

mod update_dnf;
mod quotes;
mod update_flatpak;
mod command;
mod cache_clean;
mod snapshot;
mod reboot;

fn main() {
    println!("Hello, from fedora-update!");
    snapshot_make();
    update_dnf();
    update_flatpak();
    cleanup();
    reboot_check();
    end_quote();
}