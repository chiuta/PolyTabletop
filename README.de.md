[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · **Deutsch** · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 0.9.1 · Last updated: 2026-11-26

## Was ist das?

PolyTabletop ist eine Clean-Room-Sammlung von Brett- und Tischspielen, geschrieben in Rust: 171 spielbare Spiele gegen den Computer, im Terminal oder in einem grafischen Fenster, dazu der Hot-Seat-Modus am selben Rechner und Netzwerkspiel für zwei Personen. Es gibt abstrakte Klassiker, Karten- und Würfelspiele, originale Varianten moderner Mechaniken unter eigenen Namen, kooperative Spiele mit KI-Partner, Spiele mit verdeckten Rollen, bei denen Roboter den Rest des Tisches besetzen, Party- und Wortspiele sowie Engines für Miniaturen und Sammelkartenspiele mit originalen Karten. Die Engine und die Spiele haben keine externen Abhängigkeiten; nur die grafische Oberfläche (`polytabletop-gui`) nutzt `eframe`/egui. Der Katalog führt 251 Zielspiele.

Vorstellungsseite: <https://polytabletop.chiuta.ro>. Kurzanleitung zur Installation: [docs/INSTALL.de.md](docs/INSTALL.de.md) ([English](docs/INSTALL.en.md) · [Română](docs/INSTALL.ro.md) · [Français](docs/INSTALL.fr.md) · [Italiano](docs/INSTALL.it.md) · [Español](docs/INSTALL.es.md) · [Português](docs/INSTALL.pt.md) · [Magyar](docs/INSTALL.hu.md)).

## Benutzung

- Liste und Katalog: `polytabletop list` zeigt die 171 spielbaren Spiele mit kurzen Regeln; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` zeigt den Stand aller 251 Katalogeinträge.
- Spielen: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (Standard) macht dich zu Spieler 1, `two` zu Spieler 2, `both` ist Hot-Seat an einer Tastatur, `none` lässt den Computer beide Seiten spielen. `--seed N` macht Kartenverteilung und Würfel reproduzierbar; `--record DATEI` speichert ein Protokoll.
- Im Spiel: den Zug genau so eingeben, wie `?` ihn auflistet (zum Beispiel `e2e4`, `c3xe5`, `d6`), `?` = legale Züge, `hint` = Vorschlag der KI, `q` = beenden.
- KI gegen KI: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (siehe [docs/ARENA.md](docs/ARENA.md)).
- Zwei Personen im Netzwerk: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` und `polytabletop join HOST:PORT`.
- Protokolle: `polytabletop replay DATEI [--step]`, `polytabletop export DATEI --format pgn|text|json` (PGN nur für Schach), `polytabletop import-pgn DATEI [--out PROTOKOLL]`.
- Sprachen: `polytabletop langs` listet die 8 Oberflächensprachen; `--lang xx` darf an beliebiger Stelle der Befehlszeile stehen.
- Grafische Oberfläche: `cargo run --release -p polytabletop-gui` (`-- --selftest` führt die Prüfungen ohne Bildschirm aus). Toolkit für Rollenspiele: `cargo run --release -p polytabletop-rpg`.
- Von `install.sh` installiert: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, die Man-Page und der Kurzalias `ptt` (`ptt list` = `polytabletop list`).
- Umgebungsvariablen: `POLYTABLETOP_LANG` (Sprache), `POLYTABLETOP_LEVEL` (easy, normal, hard oder expert), `POLYTABLETOP_AI_MS` (Zeitbudget der KI in Millisekunden).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # du bist Spieler 1 (Weiß)
./target/release/polytabletop play go --you two      # der Computer beginnt
./target/release/polytabletop --lang de play barbut --seed 42
```

## Spiele

171 Spiele sind jetzt spielbar, im Terminal und in der grafischen Oberfläche; der Katalog hat 251 Einträge (`polytabletop catalog` endet mit: implementiert 182, traditionelle Spiele geplant 50, eigene Varianten geplant 3, generisches Toolkit 16). Die 16 Toolkit-Einträge sind Rollenspielsysteme, bedient vom generischen Crate `polytabletop-rpg` (Würfel, Charakterbögen, Fertigkeitsproben, Dungeon-Generator, Schlachtsimulator, Journal), ohne die Regeln irgendeines Verlags. Kategorien und Anzahl der Spiele, aus `polytabletop-games/src/lib.rs`:

| Kategorie | Spiele | Beispiele (IDs) |
|---|---|---|
| Karten | 45 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda` |
| Abstrakt | 40 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane` |
| Strategie | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, dazu originale Varianten moderner Spiele |
| Partyspiele | 16 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Würfel | 11 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut` |
| Klassiker | 9 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo` |
| Familie | 7 | `orasel`, `roata`, `turnul`, `zidurile` |
| Miniaturen | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Kooperativ | 6 | `artificii`, `focare`, `strajile`, `cripta` |
| Sammelkarten (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Deduktion | 1 | `conac` |
| Wörter | 1 | `litere` |
| **Gesamt** | **171** | |

Drei Katalogeinträge bleiben als originale Varianten geplant (Twister, Campionii Distracției, Turistico). Die Kurzregeln jedes Spiels stehen in [docs/JOCURI.md](docs/JOCURI.md), der erzeugte Katalog in [docs/CATALOG.md](docs/CATALOG.md). In der grafischen Oberfläche haben 36 Spiele ein gezeichnetes Brett mit Zügen per Klick (Schach, Dame, Reversi, Vier gewinnt, Gomoku, Go, Mühle, Xiangqi, Shogi, Backgammon, Ludo und Pachisi, Hex, die Mancala-Familie und weitere); die übrigen Spiele erscheinen als Text, ihre legalen Züge als Schaltflächen.

## Sprachen

PolyTabletop spricht 8 Sprachen: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. Die Befehlszeilen- und die grafische Oberfläche sind in allen 8 übersetzt. Die Sprache wird durch `--lang xx` gewählt, dann `POLYTABLETOP_LANG`, dann das `LANG` des Systems, sonst Rumänisch. Titel und Regeln aller 171 Spiele sind in die 8 Sprachen übersetzt (`polytabletop-games/data/i18n/`, mit Rückfall auf den rumänischen Text, falls ein Eintrag fehlt); der Text innerhalb der Spiele (Karten, Inhaltsbanken, Meldungen) ist noch rumänisch. Die Zugnotation (`e2e4`, `c3-d4`) ändert sich nie. Einzelheiten: [docs/I18N.md](docs/I18N.md).

## KI

- Abstrakte Spiele nutzen Alpha-Beta-Suche mit iterativer Vertiefung, Zeitbudget, Transpositionstabelle und Ruhesuche; Spiele mit Würfeln, Karten und großem Verzweigungsfaktor nutzen MCTS (UCT). Einzelheiten in [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Die Stufen `easy`, `normal`, `hard`, `expert` skalieren das Zeitbudget und die Zahl der Simulationen (etwa 0,1x, 1x, 2,5x und 6x); bei `easy` spielt die KI manchmal absichtlich einen schwächeren Zug.
- Die KI schummelt nicht: Bewertung und Simulationen lesen nur die für den jeweiligen Spieler sichtbaren Informationen; bei Spielen mit verdeckten Händen wird der unbekannte Teil zufällig bestimmt, statt in die Hand des Gegners zu schauen.
- Verdeckte Information und Hot-Seat: Im Modus `--you both` zeigen die Spiele aus `HOTSEAT_SAFE` nur die privaten Informationen des Spielers am Zug; bei den anderen Spielen mit zufälligem verdecktem Zustand erscheint eine Warnung, und `hint` ist abgeschaltet. Jedes Spiel gehört zu einer von drei Klassen (sicher, ein Mensch gegen Roboter mit Gesamtansicht für einen zweiten Menschen, oder ohne private Information), von einem Test geprüft. Siehe [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Netzwerkspiel

`polytabletop host ID` wartet auf einen Client am Port 7878 (oder `--port P`) und spielt selbst als Spieler 1 (oder 2 bzw. zufällig mit `--side`); `polytabletop join HOST:PORT` verbindet sich. Das Textprotokoll `polytabletop/1` ist auf dem Server maßgeblich: Dort liegt die einzige Kopie der Partie, und jede Seite sieht nur ihre private Information. Wer nicht innerhalb von `--timeout` Sekunden (standardmäßig 600) zieht, verliert. Im Terminal gibt man einen Zug, `?`, `state`, `chat TEXT` oder `q` ein. Protokoll und Einzelheiten: [docs/RETEA.md](docs/RETEA.md). Das Netzwerkspiel ist über Loopback getestet (`polytabletop/tests/net_loopback.rs`), nicht in echten Netzen.

## Regelseiten

Jedes spielbare Spiel hat eine Seite mit Ziel, Vorbereitung, Zugablauf, Zugsyntax, Sonderregeln, Wertung, Spielende, Hinweisen zur KI und Abweichungen von den traditionellen Regeln: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (Spiele 1 bis 86) und [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (Spiele 87 bis 171). Die Seiten sind auf Rumänisch.

## Architektur

| Crate | Aufgabe |
|---|---|
| `polytabletop-core` | typisierter Trait `Game`, objektsicherer Trait `GameObj` (Züge als Text, für jede Oberfläche), KI (Alpha-Beta, MCTS), eigener Zufallsgenerator, i18n; ohne externe Abhängigkeiten |
| `polytabletop-games` | alle Spiele (je ein Modul), `playable()`, `HOTSEAT_SAFE`, erzeugter Katalog; ohne externe Abhängigkeiten |
| `polytabletop-rpg` | generisches Toolkit für Rollenspiele; ohne externe Abhängigkeiten |
| `polytabletop-gui` | grafische Oberfläche egui/eframe; das einzige Crate mit externen Abhängigkeiten, außerhalb des Standard-Builds |
| `polytabletop` | Befehlszeilenoberfläche, Arena, Netzwerk, Protokolle |

Ein neues Spiel ist eine Struktur, die `Game` implementiert (legale Züge, Anwenden, Ergebnis, Bewertung, Darstellung), plus ein Eintrag in `playable()`; jede Oberfläche bekommt es gratis. Siehe [docs/ARHITECTURA.md](docs/ARHITECTURA.md) und [docs/GUI.md](docs/GUI.md) (Dokumente auf Rumänisch).

## Bauen, Testen und CI

- Bauen: `cargo build --release` (Rust, aktuelle stabile Toolchain). Die grafische Oberfläche gehört nicht zum Standard-Build: `cargo build --release -p polytabletop-gui`; unter Linux braucht sie die X11- oder Wayland-Entwicklungsbibliotheken.
- Testen: `cargo test --release` (Schach-Perft 20 / 400 / 8.902 / 197.281, Sonderregeln, vollständige Zufallspartien für jedes Spiel, KI gegen Zufallszüge, Ansichtsprüfung, Katalogkonsistenz, Netzwerk über Loopback).
- Vor einem Pull-Request: `cargo fmt --all` und `cargo clippy --release --all-targets -- -D warnings`.
- Die CI (`.github/workflows/ci.yml`) führt fmt, clippy und die Tests unter Ubuntu 26.04 aus, baut dann die grafische Oberfläche und startet `polytabletop-gui --selftest`. Ein Tag `v*` löst `release.yml` aus, das Archive für Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) mit SHA-256-Summen baut; `install.sh` installiert sie von `chiuta/PolyTabletop`. Ein `Dockerfile` (nur CLI und RPG) liegt bei.

## Clean-Room und Marken

- Die Regeln stammen nur aus öffentlichen Beschreibungen der Spiele; es werden weder Code, Texte, Grafiken noch Daten kommerzieller Produkte verwendet, und die öffentlichen Quellen stehen im Kopf jeder Spieldatei.
- Die Mechanik eines Spiels ist frei, sein Name, seine Grafik und seine Texte nicht: Die Spiele tragen generische Namen („Reversi“, „Vier in einer Reihe“) oder sind bei modernen Markenspielen originale Varianten mit eigenen Titeln, Themen und Inhalten.
- Keine Marken in IDs, Titeln, Kartennamen oder Inhaltsbanken. Markennamen erscheinen nur im Katalog, als Planungsziele.
- Rollenspiele, Miniaturen und Sammelkarten sind generische Toolkits (Würfel, Bögen, originale Decks), nie eine Kopie der Regeln oder Inhalte eines Verlags.

## Grenzen

- Die grafische Oberfläche wurde nur ohne Bildschirm getestet (`polytabletop-gui --selftest`); sie wurde nicht visuell auf einem echten Bildschirm geprüft.
- Die Stärke der KI wurde gegen Zufallszüge und zwischen Zeitbudgets gemessen, nicht gegen menschliche Spieler; einige Spiele haben bekannte Grenzen (zum Beispiel ist die Siegquote von `macao` und `hruba` im Changelog festgehalten).
- Die aus dem Gedächtnis geschriebenen Spielregeln wurden mit öffentlichen Quellen verglichen, nicht mit jedem offiziellen Regelwerk, und die Spiele wurden nicht von externen Spielern getestet.
- Das Netzwerkspiel ist nur über Loopback getestet.
- Spieltitel und Regeln sind übersetzt, aber der Text innerhalb der Spiele (Karten, Inhaltsbanken, Meldungen) ist noch rumänisch; die Übersetzungen wurden nicht von Muttersprachlern geprüft.
- Die 16 Rollenspieleinträge werden von einem generischen Toolkit bedient, nicht von Implementierungen dieser Systeme.

## Mitwirken und Änderungsverlauf

Beiträge sind willkommen im Rahmen der Clean-Room-Regeln: siehe [CONTRIBUTING.md](CONTRIBUTING.md) (ein Spiel hinzufügen, Tests, Checkliste für Pull-Requests). Versionsverlauf: [CHANGELOG.md](CHANGELOG.md).

## Lizenz

In `Cargo.toml` als `MIT OR Apache-2.0` angegeben; Volltexte in [LICENSE-MIT](LICENSE-MIT) und [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Danksagung und Autor

Autor: Alexio (Alexandru-Ionuț Chiuță), Teil der Reihe „Fără Frontiere“ (Ohne Grenzen) trade-freier Anwendungen. Kontakt: alexio@trom.tf. Vorstellungsseite: <https://polytabletop.chiuta.ro>.
