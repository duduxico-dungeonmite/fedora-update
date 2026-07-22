use crate::cache_clean::cleanup;
use crate::quotes::end_quote;
use crate::snapshot::snapshot_make;
use crate::update_dnf::update_dnf;
use crate::update_flatpak::update_flatpak;
use colored::Colorize;
mod update_dnf;
mod quotes;
mod update_flatpak;
mod command;
mod cache_clean;
mod snapshot;

fn main() {
    println!("{}", "Hello, from fedora-update!".purple());
    snapshot_make();
    update_dnf();
    update_flatpak();
    cleanup();
    end_quote();
}

// fedora-update
// Copyright (C) 2026 Eduardo Francisco Bertoli Mancia
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//
// See <https://www.gnu.org/licenses/>.
