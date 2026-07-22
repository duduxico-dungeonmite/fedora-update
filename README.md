# fedora-update
Fedora equivalent of "arch-update"!


"fedora-update" is basically something akin to "arch-update" from Arch Linux, but you guessed it: for Fedora!

It contains:
- Snapper snapshot creation before the update
- Updates to DNF and Flatpak packages
- Package cache cleanup
- Checks for reboots if needed!

## Requirements

- Fedora (This was built using Fedora 44 KDE)
- Snapper
- Btrfs
- Flatpak (optional)
- Cargo
- Rust

## Installation

On your Linux terminal, use these commands:

`git clone https://github.com/duduxico-dungeonmite/fedora-update.git`

`cd fedora-update`

`cargo build --release`

`sudo cp target/release/fedora-update /usr/local/bin/`

<img width="1920" height="611" alt="image" src="https://github.com/user-attachments/assets/05261381-a329-43ab-acc2-5e6884955e7a" />

fedora-update working on my machine!



## Credits

fedora-update is heavily inspired by Arch Linux maintainer Robin Candau (Antiz96)'s tool "arch-update" (https://github.com/Antiz96/arch-update). This project is not affiliated with Arch Linux, nor Antiz96.
