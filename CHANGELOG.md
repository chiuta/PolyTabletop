# Changelog

## v0.9
- Project renamed from MASA to PolyTabletop; 171 playable games (201 catalog entries).
- Interface in 8 languages (en ro fr it es pt de hu) for the CLI and the GUI: `--lang`, `POLYTABLETOP_LANG`; READMEs and install guides in all 8.
- AI levels `--level easy|normal|hard|expert` (`POLYTABLETOP_LEVEL`).
- `polytabletop arena`, `bench`, `ladder` (AI against AI); `host` / `join` for two-player network play; `--record`, `replay`, `export` (pgn for chess, text, json) and `import-pgn`.
- `install.sh` (installs the `ptt` alias), man page, shell completions, Dockerfile; CI and release workflows on Ubuntu 26.04.
- Titles and rules of all 171 games translated into the 8 languages (text inside the games is still Romanian).

## v0.8
- 10 new games: Coloane, Bile, Patru, Cupe, Poliedre, Inimi, Pica, Prinde, Whist (plus shared trick-taking helpers).
- `--seed N` makes deals and dice reproducible (CLI and GUI).
- GUI: 16 new graphical boards; 36 games with a board in total.
- AI: Tigri si capre re-evaluated; strength test `w6_strength` (ignored by default).
- Not implemented: Risca (no verifiable public rules).

## v0.7
- Correct hot-seat mode for 46 games.
- Better gomoku AI; fixes for expeditii; macao ~82%, hruba ~23%.
- 21 new games (Hex, Amazoane, Linii, Nim, Coridor, Drum, Konane, Fanorona, Tigri, Surakarta, Hasami, ... Cassino, Poker de zaruri).
- GUI: boards for Shogi, Table, Ludo/Pachisi, last-move highlight, undo.
- polytabletop-rpg: battle simulator, character generator, generators, balancing, hex map, Markdown export.
- Documentation in `docs/`; general QA tests (`z4_qa`).
- Fixed: chess (repetition with en passant, insufficient material), sideral (shield deadlock).

## v0.6
- New search engine (iterative deepening, transposition table, quiescence); time budget via `POLYTABLETOP_AI_MS`.
- Hot-seat mode (`--you both`) no longer shows Player 1's hand in safe-marked games; warning for the others.
- New games: Rezervatia, Hotar, Codru, Sideral, Ceasornic, Coroana.
- Not implemented: Twister, Campionii Distractiei, Turistico.

## v0.1 - v0.5
- Typed `Game` trait, `GameObj`, alpha-beta and MCTS (UCT) AI; command-line front-end (`list`, `catalog`, `play`).
- First waves of abstract, dice, card and classic games; clean-room policy; generated catalog of 176 entries.
- First egui GUI (`polytabletop-gui`) with graphical boards for chess, draughts, reversi, connect four, gomoku, go, morris, xiangqi.
- `polytabletop-rpg` toolkit (dice, sheets, ability checks, dungeon, journal).
