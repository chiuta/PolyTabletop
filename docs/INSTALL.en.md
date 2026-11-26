# Install and quick start

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## What it is

**PolyTabletop** is a collection of table-top games (chess, go, draughts, card, dice and many more) written from scratch in Rust, playable in the terminal, in a graphical interface, and over a local network. The Romanian text is the reference one; this page is a short guide.

## Install

- **From a release archive.** Download the archive for your platform from the project's Releases page, unpack it anywhere (it needs no installer and no external libraries) and run the `polytabletop` executable from that folder.
- **With the install script (Linux and macOS).** `bash install.sh [version-tag]` (script in the repository `chiuta/PolyTabletop`) downloads the latest release archive for your platform, checks its SHA-256 sum and installs `polytabletop`, `polytabletop-gui` and `polytabletop-rpg` in `~/.local/bin` (change it with `PREFIX=...`), plus the man page and the short alias `ptt`. On Windows, download the `.zip` archive from Releases.
- **From source with cargo** (Rust, current stable toolchain):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Graphical interface (optional): `cargo run --release -p polytabletop-gui`.

## First game

`polytabletop list` shows the games; `polytabletop play <game>` starts one. Type a move (e.g. `e2e4`), `?` = legal moves, `hint` = the AI's suggestion, `q` = quit.

## Hot-seat or against the AI

```
./target/release/polytabletop play chess --you one     # you play Player 1 (default)
./target/release/polytabletop play chess --you two     # you play Player 2 (the computer starts)
./target/release/polytabletop play chess --you both    # hot-seat: both players at the same keyboard
./target/release/polytabletop play go --you two
```

## Reproducible games (`--seed`)

`--seed N` makes the card deal and the dice reproducible: the same number gives the same game.

```
./target/release/polytabletop play barbut --seed 42
```

## LAN: host and join

On a local network, one machine hosts the game and the other connects to it. Both machines must be able to reach each other (firewall open on the chosen port).

```
./target/release/polytabletop host chess               # one player hosts the game
./target/release/polytabletop join 192.168.1.20:7878   # the other joins using the host's address and port
```

## Languages (`--lang`)

The interface speaks 8 languages: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (chosen by `--lang`, then `POLYTABLETOP_LANG`, then the system `LANG`; otherwise `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

More: [README](../README.md).
