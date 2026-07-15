# fedora-update
Simple "arch-update" Fedora equivalent!

## This was originally a vibe-coded tool. Changes to the code for polish and general upgrades are more than welcome!

"fedora-update" is basically something akin to "arch-update" from Arch Linux.

It contains:
- Snapper snapshot creation before the update
- Updates to DNF and Flatpak packages
- Package cache clearence
- Checks for reboots if needed!

## Requirements

- Fedora (This was built using Fedora 44 KDE)
- Snapper
- Btrfs
- Flatpak (optional)

## Installation

git clone https://github.com/duduxico-dungeonmite/fedora-update.git

cd fedora-update

chmod +x fedora-update
sudo cp fedora-update /usr/local/bin/

<img width="1920" height="656" alt="image" src="https://github.com/user-attachments/assets/59c21062-dc81-40d9-a73b-a3c291f0363b" />
^ the tool working on my PC!
