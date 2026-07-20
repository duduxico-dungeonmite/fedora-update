use crate::cache_clean::cleanup;
use crate::quotes::end_quote;
use crate::update_dnf::update_dnf;
use crate::update_flatpak::update_flatpak;

mod update_dnf;
mod quotes;
mod update_flatpak;
mod command;
mod cache_clean;

fn main() {
    println!("Hello, from fedora-update!");
    update_dnf();
    update_flatpak();
    cleanup();
    end_quote();
}
