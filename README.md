**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 0.9.1 · Last updated: 2026-11-26

## What it is

PolyTabletop is a clean-room collection of table-top games written in Rust: 171 playable games against the computer, in the terminal or in a graphical window, plus hot-seat and two-player network play. Abstract classics, card and dice games, original variants of modern mechanics under names of their own, cooperative games with an AI partner, hidden-role games where bots fill the rest of the table, party and word games, miniatures and collectible-card engines with original cards. The engine and the games have no external dependencies; only the graphical interface (`polytabletop-gui`) uses `eframe`/egui. The catalog tracks 251 target games.

Presentation page: <https://polytabletop.chiuta.ro>. Short install guide in your language: [docs/INSTALL.en.md](docs/INSTALL.en.md) ([Română](docs/INSTALL.ro.md) · [Français](docs/INSTALL.fr.md) · [Italiano](docs/INSTALL.it.md) · [Español](docs/INSTALL.es.md) · [Português](docs/INSTALL.pt.md) · [Deutsch](docs/INSTALL.de.md) · [Magyar](docs/INSTALL.hu.md)).

## Use

- List and catalog: `polytabletop list` shows the 171 playable games with their rules in short; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` shows the status of all 251 catalog entries.
- Play: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (default) makes you Player 1, `two` Player 2, `both` is hot-seat at one keyboard, `none` lets the computer play both sides. `--seed N` makes deals and dice reproducible; `--record FILE` saves a transcript.
- In a game: type the move exactly as `?` lists it (for example `e2e4`, `c3xe5`, `d6`), `?` = legal moves, `hint` = the AI's suggestion, `q` = quit.
- AI against AI: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (see [docs/ARENA.md](docs/ARENA.md)).
- Two people over the network: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` and `polytabletop join HOST:PORT`.
- Transcripts: `polytabletop replay FILE [--step]`, `polytabletop export FILE --format pgn|text|json` (PGN only for chess), `polytabletop import-pgn FILE [--out TRANSCRIPT]`.
- Languages: `polytabletop langs` lists the 8 interface languages; `--lang xx` can be given anywhere on the command line.
- Graphical interface: `cargo run --release -p polytabletop-gui` (`-- --selftest` runs the headless checks). RPG toolkit: `cargo run --release -p polytabletop-rpg`.
- Installed by `install.sh`: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, the man page and the short alias `ptt` (`ptt list` = `polytabletop list`).
- Environment variables: `POLYTABLETOP_LANG` (language), `POLYTABLETOP_LEVEL` (easy, normal, hard or expert), `POLYTABLETOP_AI_MS` (time budget of the AI in milliseconds).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # you are Player 1 (white)
./target/release/polytabletop play go --you two      # the computer starts
./target/release/polytabletop --lang de play barbut --seed 42
```

## Games

171 games can be played now, in the terminal and in the graphical interface; the catalog has 251 entries (`polytabletop catalog` ends with: implemented 182, traditional games planned 50, own variants planned 3, generic toolkit 16). The 16 toolkit entries are role-playing systems, served by the generic `polytabletop-rpg` crate (dice, sheets, ability checks, dungeon generator, battle simulator, journal) without any publisher's rules. Categories and counts, from `polytabletop-games/src/lib.rs`:

| Category | Games | Examples (ids) |
|---|---|---|
| Cards | 45 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda` |
| Abstract | 40 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane` |
| Strategy | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, plus original variants of modern designs |
| Party | 16 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Dice | 11 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut` |
| Classic | 9 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo` |
| Family | 7 | `orasel`, `roata`, `turnul`, `zidurile` |
| Miniatures | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Cooperative | 6 | `artificii`, `focare`, `strajile`, `cripta` |
| Collectible cards (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Deduction | 1 | `conac` |
| Words | 1 | `litere` |
| **Total** | **171** | |

Three catalog entries remain planned as original variants (Twister, Campionii Distracției, Turistico). Short rules for every game are in [docs/JOCURI.md](docs/JOCURI.md), the generated catalog in [docs/CATALOG.md](docs/CATALOG.md). In the graphical interface, 36 games have a drawn board with clickable moves (chess, draughts, reversi, four in a row, gomoku, go, morris, xiangqi, shogi, backgammon, ludo and pachisi, hex, the mancala family and more); the other games are shown as text with their legal moves as buttons.

## Languages

PolyTabletop speaks 8 languages: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. The command-line and graphical interfaces are translated in all 8. The language is chosen by `--lang xx`, then `POLYTABLETOP_LANG`, then the system `LANG`, otherwise Romanian. The titles and rules of all 171 games are translated into the 8 languages (`polytabletop-games/data/i18n/`, with fallback to the Romanian text if an entry is missing); the text inside the games (cards, content banks, prompts) is still in Romanian. Move notation (`e2e4`, `c3-d4`) never changes. Details: [docs/I18N.md](docs/I18N.md).

## AI

- Abstract games use alpha-beta search with iterative deepening, a time budget, a transposition table and quiescence; games with dice, cards and a large branching factor use MCTS (UCT). Details in [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Levels `easy`, `normal`, `hard`, `expert` scale the time budget and the number of simulations (about 0.1x, 1x, 2.5x and 6x); at `easy` the AI sometimes plays a weaker move on purpose.
- The AI does not cheat: its evaluation and simulations read only the information visible to the player it plays for; games with hidden hands guess the unknown part at random instead of looking at the opponent's hand.
- Hidden information and hot-seat: in `--you both` mode, games listed in `HOTSEAT_SAFE` show only the private information of the player to move; for the other games with random hidden state a warning appears and `hint` is disabled. Each game is one of three kinds (safe, single human against bots with a full view for a second human, or without private information), checked by a test. See [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Network play

`polytabletop host ID` waits for one client on port 7878 (or `--port P`) and plays as Player 1 (or 2 or random with `--side`); `polytabletop join HOST:PORT` connects. The text protocol `polytabletop/1` is authoritative on the server: the only copy of the game lives there, and each side sees only its own private information. Whoever does not move within `--timeout` seconds (600 by default) loses. In the terminal you type a move, `?`, `state`, `chat TEXT` or `q`. Protocol and details: [docs/RETEA.md](docs/RETEA.md). The network play is tested over loopback (`polytabletop/tests/net_loopback.rs`), not across real networks.

## Rules pages

Every playable game has a page with the goal, setup, turn structure, move syntax, special rules, scoring, end of game, AI notes and differences from traditional rules: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (games 1 to 86) and [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (games 87 to 171). The pages are in Romanian.

## Architecture

| Crate | Role |
|---|---|
| `polytabletop-core` | typed `Game` trait, object-safe `GameObj` (moves as text, for any interface), AI (alpha-beta, MCTS), own random generator, i18n; no external dependencies |
| `polytabletop-games` | all games (one module each), `playable()`, `HOTSEAT_SAFE`, generated catalog; no external dependencies |
| `polytabletop-rpg` | generic role-playing toolkit; no external dependencies |
| `polytabletop-gui` | egui/eframe graphical interface; the only crate with external dependencies, outside the default build |
| `polytabletop` | command-line interface, arena, network, transcripts |

A new game is a struct that implements `Game` (legal moves, apply, outcome, evaluation, display) plus one entry in `playable()`; every front end gets it for free. See [docs/ARHITECTURA.md](docs/ARHITECTURA.md) and [docs/GUI.md](docs/GUI.md) (these documents are in Romanian).

## Building, testing and CI

- Build: `cargo build --release` (Rust, current stable toolchain). The graphical interface is not a default member: `cargo build --release -p polytabletop-gui`; on Linux it needs the X11 or Wayland development libraries.
- Test: `cargo test --release` (chess perft 20 / 400 / 8,902 / 197,281, special rules, complete random games for every game, AI against random moves, viewer audit, catalog consistency, network over loopback).
- Before a pull request: `cargo fmt --all` and `cargo clippy --release --all-targets -- -D warnings`.
- CI (`.github/workflows/ci.yml`) runs fmt, clippy and the tests on Ubuntu 26.04, and builds the GUI and runs `polytabletop-gui --selftest`. A tag `v*` triggers `release.yml`, which builds archives for Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) with SHA-256 sums; `install.sh` installs them from `chiuta/PolyTabletop`. A `Dockerfile` (CLI and RPG only) is included.

## Clean-room and trademarks

- Rules come only from public descriptions of the games; no code, text, graphics or data of commercial products are used, and the public sources are noted in the header of each game file.
- A game's mechanics are free, its name, art and text are not: games carry generic names ("Reversi", "Four in a Row") or, for modern branded games, original variants with their own titles, themes and content.
- No trademarks in ids, titles, card names or content banks. Brand names appear only in the catalog, as planning targets.
- Role-playing, miniatures and collectible cards are generic toolkits (dice, sheets, original decks), never a copy of a publisher's rules or content.

## Limits

- The graphical interface was tested only headless (`polytabletop-gui --selftest`); it was not checked visually on a real screen.
- AI strength was measured against random moves and between time budgets, not against human players; some designs have known limits (for example the win rate of `macao` and `hruba` is documented in the changelog).
- The rules of games written from memory were compared with public sources, not with every official rulebook, and the games have not been playtested by outside players.
- Network play is tested over loopback only.
- The titles and rules of the games are translated, but the text inside the games (cards, content banks, prompts) is still Romanian; the translations were not proofread by native speakers.
- The 16 role-playing entries are served by a generic toolkit, not by implementations of those systems.

## Contributing and changelog

Contributions are welcome under the clean-room rules: see [CONTRIBUTING.md](CONTRIBUTING.md) (how to add a game, tests, pull-request checklist). Version history: [CHANGELOG.md](CHANGELOG.md).

## License

Declared in `Cargo.toml` as `MIT OR Apache-2.0`; full texts in [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Credits

Author: Alexio (Alexandru-Ionuț Chiuță), part of the "Fără Frontiere" (Without Borders) series of trade-free applications. Contact: alexio@trom.tf. Presentation page: <https://polytabletop.chiuta.ro>.
