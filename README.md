# fedora-update

## The Fedora equivalent of "arch-update" from Arch Linux.

"fedora-update" is essentially similar to "arch-update" from Arch Linux, but for Fedora.

### Check out the [Wiki!](https://github.com/duduxico-dungeonmite/fedora-update/wiki)

### It contains:

- Snapper snapshot creation before the update

- Updates to DNF and Flatpak packages

- Package cache cleanup

- Checks whether a reboot is required after updating

- Privilege escalation (with sudo and doas)

## Requirements

- Fedora (This was built using Fedora 44 KDE)

- Snapper (for snapshots, optional if you do not want it)

- Btrfs

- dnf-utils (for needs-restarting, the restarting checker)

- Flatpak (optional)

- Cargo (only if compiling from source)

- Rust (only if compiling from source)

## Installation

### Installing from Fedora COPR:

`sudo dnf copr enable duduxico/fedora-update`

`sudo dnf install fedora-update`

### Compiling from source:

`git clone https://github.com/duduxico-dungeonmite/fedora-update.git`

`cd fedora-update`

`cargo build --release`

`sudo cp target/release/fedora-update /usr/local/bin/`

<img width="1920" height="611" alt="image" src="https://github.com/user-attachments/assets/05261381-a329-43ab-acc2-5e6884955e7a" />

fedora-update working on my machine!

## Credits

fedora-update is heavily inspired by Arch Linux maintainer Robin Candau (Antiz96)'s tool ["arch-update"](https://github.com/Antiz96/arch-update). This project is not affiliated with Arch Linux, nor Antiz96.

# Licensing
This program is licensed under the GNU General Public License v3.0. It is written in Rust and contains elements of the Rust Standard Library, which is dual-licensed under the MIT License and the Apache License 2.0.
