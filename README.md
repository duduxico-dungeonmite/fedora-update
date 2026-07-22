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

git clone https://github.com/duduxico-dungeonmite/fedora-update.git

cd fedora-update

chmod +x fedora-update
sudo cp fedora-update /usr/local/bin/

<img width="1920" height="656" alt="image" src="https://github.com/user-attachments/assets/59c21062-dc81-40d9-a73b-a3c291f0363b" />
^ the tool working on my PC!

## Credits

fedora-update is heavily inspired by Arch Linux maintainer Robin Candau (Antiz96)'s tool "arch-update" (https://github.com/Antiz96/arch-update). This project is not affiliated with Arch Linux, nor Antiz96.
