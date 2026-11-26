# Installation und Schnellstart

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## Was ist das?

**PolyTabletop** ist eine Sammlung von Brett- und Gesellschaftsspielen (Schach, Go, Dame, Karten-, Würfelspiele und vieles mehr), von Grund auf in Rust geschrieben und spielbar im Terminal, in einer grafischen Oberfläche und im lokalen Netzwerk. Der rumänische Text ist maßgeblich; diese Seite ist eine Kurzanleitung.

## Installation

- **Aus einem Release-Archiv.** Lade das Archiv für deine Plattform von der Releases-Seite des Projekts herunter, entpacke es beliebig (kein Installer und keine externen Bibliotheken nötig) und starte die ausführbare Datei `polytabletop` aus diesem Ordner.
- **Mit dem Installationsskript (Linux und macOS).** `bash install.sh [Versions-Tag]` (Skript im Repository `chiuta/PolyTabletop`) lädt das neueste Release-Archiv für deine Plattform herunter, prüft die SHA-256-Summe und installiert `polytabletop`, `polytabletop-gui` und `polytabletop-rpg` nach `~/.local/bin` (änderbar mit `PREFIX=...`), dazu die Man-Page und den Kurzalias `ptt`. Unter Windows lädst du das `.zip`-Archiv von Releases herunter.
- **Aus dem Quellcode mit cargo** (Rust, aktuelle stabile Toolchain):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Grafische Oberfläche (optional): `cargo run --release -p polytabletop-gui`.

## Das erste Spiel

`polytabletop list` zeigt die Spiele; `polytabletop play <Spiel>` startet eines. Gib einen Zug ein (z. B. `e2e4`), `?` = legale Züge, `hint` = Vorschlag der KI, `q` = Beenden.

## Zu zweit am selben Rechner oder gegen die KI

```
./target/release/polytabletop play chess --you one     # du spielst Spieler 1 (Standard)
./target/release/polytabletop play chess --you two     # du spielst Spieler 2 (der Computer beginnt)
./target/release/polytabletop play chess --you both    # Hot-Seat: beide Spieler an derselben Tastatur
./target/release/polytabletop play go --you two
```

## Reproduzierbare Partien (`--seed`)

`--seed N` macht Kartengeben und Würfel reproduzierbar: dieselbe Zahl ergibt dieselbe Partie.

```
./target/release/polytabletop play barbut --seed 42
```

## Lokales Netzwerk: hosten und beitreten

Im lokalen Netzwerk hostet ein Rechner die Partie und der andere verbindet sich mit ihm. Beide Rechner müssen sich erreichen können (Firewall auf dem gewählten Port geöffnet).

```
./target/release/polytabletop host chess               # ein Spieler hostet die Partie
./target/release/polytabletop join 192.168.1.20:7878   # der andere tritt mit Adresse und Port des Hosts bei
```

## Sprachen (`--lang`)

Die Oberfläche spricht 8 Sprachen: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (gewählt per `--lang`, dann `POLYTABLETOP_LANG`, dann dem System-`LANG`; sonst `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

Mehr dazu: [README](../README.de.md).
