# Installazione e avvio rapido

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## Che cos'è

**PolyTabletop** è una raccolta di giochi da tavolo (scacchi, go, dama, giochi di carte, di dadi e molti altri) scritti da zero in Rust, da giocare nel terminale, in un'interfaccia grafica e in rete locale. Il testo in rumeno è quello di riferimento; questa pagina è una guida breve.

## Installazione

- **Da un archivio di rilascio.** Scarica l'archivio per la tua piattaforma dalla pagina Releases del progetto, estrailo dove vuoi (non servono installer né librerie esterne) ed esegui l'eseguibile `polytabletop` da quella cartella.
- **Con lo script di installazione (Linux e macOS).** `bash install.sh [tag-versione]` (script nel repository `chiuta/PolyTabletop`) scarica l'ultimo archivio di rilascio per la tua piattaforma, verifica la somma SHA-256 e installa `polytabletop`, `polytabletop-gui` e `polytabletop-rpg` in `~/.local/bin` (modificabile con `PREFIX=...`), più la pagina di manuale e l'alias breve `ptt`. Su Windows scarica l'archivio `.zip` da Releases.
- **Dai sorgenti con cargo** (Rust, toolchain stabile recente):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Interfaccia grafica (facoltativa): `cargo run --release -p polytabletop-gui`.

## Prima partita

`polytabletop list` mostra i giochi; `polytabletop play <gioco>` ne avvia uno. Scrivi la mossa (es. `e2e4`), `?` = mosse legali, `hint` = suggerimento dell'IA, `q` = uscita.

## Sullo stesso computer o contro l'IA

```
./target/release/polytabletop play chess --you one     # giochi come Giocatore 1 (predefinito)
./target/release/polytabletop play chess --you two     # giochi come Giocatore 2 (comincia il computer)
./target/release/polytabletop play chess --you both    # a turno sullo stesso computer: entrambi i giocatori alla stessa tastiera
./target/release/polytabletop play go --you two
```

## Partite riproducibili (`--seed`)

`--seed N` rende riproducibili la distribuzione delle carte e i dadi: lo stesso numero dà la stessa partita.

```
./target/release/polytabletop play barbut --seed 42
```

## Rete locale: ospitare e unirsi

In rete locale, un computer ospita la partita e l'altro vi si collega. I due computer devono potersi raggiungere (firewall aperto sulla porta scelta).

```
./target/release/polytabletop host chess               # un giocatore ospita la partita
./target/release/polytabletop join 192.168.1.20:7878   # l'altro si unisce con l'indirizzo e la porta dell'host
```

## Lingue (`--lang`)

L'interfaccia parla 8 lingue: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (scelta con `--lang`, poi `POLYTABLETOP_LANG`, poi il `LANG` di sistema; altrimenti `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

Per saperne di più: [README](../README.it.md).
