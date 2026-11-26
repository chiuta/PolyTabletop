# Installation et démarrage rapide

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## De quoi s'agit-il

**PolyTabletop** est une collection de jeux de société (échecs, go, dames, jeux de cartes, de dés et bien d'autres) écrits de zéro en Rust, jouables dans le terminal, dans une interface graphique et en réseau local. Le texte roumain fait référence ; cette page est un guide succinct.

## Installation

- **Depuis une archive de version.** Téléchargez l'archive correspondant à votre plateforme sur la page Releases du projet, décompressez-la où vous voulez (aucun installateur ni bibliothèque externe n'est nécessaire) et lancez l'exécutable `polytabletop` depuis ce dossier.
- **Avec le script d'installation (Linux et macOS).** `bash install.sh [étiquette-de-version]` (script du dépôt `chiuta/PolyTabletop`) télécharge la dernière archive de version pour votre plateforme, vérifie sa somme SHA-256 et installe `polytabletop`, `polytabletop-gui` et `polytabletop-rpg` dans `~/.local/bin` (modifiable avec `PREFIX=...`), ainsi que la page de manuel et l'alias court `ptt`. Sous Windows, téléchargez l'archive `.zip` sur Releases.
- **Depuis les sources avec cargo** (Rust, chaîne d'outils stable récente) :

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Interface graphique (facultative) : `cargo run --release -p polytabletop-gui`.

## Première partie

`polytabletop list` affiche les jeux ; `polytabletop play <jeu>` en lance un. Saisissez un coup (ex. `e2e4`), `?` = coups légaux, `hint` = suggestion de l'IA, `q` = quitter.

## À deux sur le même poste ou contre l'IA

```
./target/release/polytabletop play chess --you one     # vous jouez le Joueur 1 (par défaut)
./target/release/polytabletop play chess --you two     # vous jouez le Joueur 2 (l'ordinateur commence)
./target/release/polytabletop play chess --you both    # à deux sur le même clavier : les deux joueurs devant le même poste
./target/release/polytabletop play go --you two
```

## Parties reproductibles (`--seed`)

`--seed N` rend reproductibles la distribution des cartes et les dés : le même nombre donne la même partie.

```
./target/release/polytabletop play barbut --seed 42
```

## Réseau local : héberger et rejoindre

En réseau local, une machine héberge la partie et l'autre s'y connecte. Les deux machines doivent pouvoir se joindre (pare-feu ouvert sur le port choisi).

```
./target/release/polytabletop host chess               # un joueur héberge la partie
./target/release/polytabletop join 192.168.1.20:7878   # l'autre la rejoint avec l'adresse et le port de l'hôte
```

## Langues (`--lang`)

L'interface parle 8 langues : `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (choisie par `--lang`, puis `POLYTABLETOP_LANG`, puis le `LANG` du système ; sinon `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

Pour en savoir plus : [README](../README.fr.md).
