# fedora-update | WORK IN PROGRESS, PLEASE CHECK `main` FOR THE NORMAL, WORKING VERSION OF fedora-update!
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
- Rust
- Cargo

## Installation

This branch is a work-in-progress Rust rewrite and isn't ready to be installed/built yet.
Check the `main` branch for the working Bash version.

## Credits

fedora-update is heavily inspired by Arch Linux maintainer Robin Candau (Antiz96)'s tool "arch-update" (https://github.com/Antiz96/arch-update). This project is not affiliated with Arch Linux, nor Antiz96.
