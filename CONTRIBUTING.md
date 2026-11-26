# Contributing to PolyTabletop

## Clean-room policy
* Rules come only from public descriptions of games. Never copy code, text, art or data from commercial products or their rulebooks.
* A game's mechanics are free to implement; its **name, art and text** are not. Use generic names ("Reversi", not "Othello") or, for branded modern games, an **original variant with its own name, theme and content**.
* **No trademarks** in ids, titles, card names or banks of content. Do not use publisher names or logos.
* RPG/miniature/TCG material stays generic (dice, sheets, own decks); no rules or content from any publisher.
* Note the public sources of the rules in the header comment of the game file.

## How to add a game
1. Create `polytabletop-games/src/<game>.rs` implementing the typed `Game` trait from `polytabletop-core` (legal moves, apply, outcome, evaluation, display). See `docs/ARHITECTURA.md`.
2. Add one `mod` line and one entry in `playable()` in `polytabletop-games/src/lib.rs` (id, title, rules summary). Hidden-information games must show only Player 1's hand; add the id to `HOTSEAT_SAFE` only if hot-seat is safe.
3. Add the entry to `tools/data` and regenerate the catalog: `python3 -I tools/gen_catalog.py`.
4. Write tests: random full games, legal-move sanity, special rules, AI vs. random.
5. Regenerate completions: `python3 -I tools/gen_completions.py`.

## Before a pull request
```
cargo fmt --all
cargo clippy --release --all-targets -- -D warnings
cargo test --release
```
`polytabletop-gui` is not a default member; build it with `cargo build --release -p polytabletop-gui` if you touch it. Commit messages: short imperative summary.
