[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 1.0.0 · Last updated: 2026-11-28

## Che cos'è

PolyTabletop è una raccolta clean-room di giochi da tavolo scritta in Rust: 240 giochi giocabili contro il computer, nel terminale o in una finestra grafica, più la modalità hot-seat sullo stesso computer e il gioco in rete per due persone. Ci sono classici astratti, giochi di carte e di dadi, varianti originali di meccaniche moderne con nomi propri, giochi cooperativi con un partner IA, giochi a ruoli nascosti in cui i robot completano il tavolo, giochi di società e di parole, motori per miniature e per carte collezionabili con carte originali. Il motore e i giochi non hanno dipendenze esterne; solo l'interfaccia grafica (`polytabletop-gui`) usa `eframe`/egui. Il catalogo traccia 1200 giochi obiettivo.

Pagina di presentazione: <https://polytabletop.chiuta.ro>. Guida rapida all'installazione: [docs/INSTALL.it.md](docs/INSTALL.it.md) ([English](docs/INSTALL.en.md) · [Română](docs/INSTALL.ro.md) · [Français](docs/INSTALL.fr.md) · [Español](docs/INSTALL.es.md) · [Português](docs/INSTALL.pt.md) · [Deutsch](docs/INSTALL.de.md) · [Magyar](docs/INSTALL.hu.md)).

## Uso

- Elenco e catalogo: `polytabletop list` mostra i 240 giochi giocabili con le regole in breve; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` mostra lo stato delle 1200 voci del catalogo.
- Giocare: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (predefinito) ti rende Giocatore 1, `two` Giocatore 2, `both` è l'hot-seat su un'unica tastiera, `none` lascia giocare al computer entrambe le parti. `--seed N` rende riproducibili la distribuzione delle carte e i dadi; `--record FILE` salva una trascrizione.
- In partita: scrivi la mossa esattamente come la elenca `?` (per esempio `e2e4`, `c3xe5`, `d6`), `?` = mosse legali, `hint` = suggerimento dell'IA, `q` = esci.
- IA contro IA: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (vedi [docs/ARENA.md](docs/ARENA.md)).
- Due persone in rete: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` e `polytabletop join HOST:PORTA`.
- Trascrizioni: `polytabletop replay FILE [--step]`, `polytabletop export FILE --format pgn|text|json` (PGN solo per gli scacchi), `polytabletop import-pgn FILE [--out TRASCRIZIONE]`.
- Lingue: `polytabletop langs` elenca le 8 lingue dell'interfaccia; `--lang xx` può stare in qualsiasi punto della riga di comando.
- Interfaccia grafica: `cargo run --release -p polytabletop-gui` (`-- --selftest` esegue le verifiche senza schermo). Kit per giochi di ruolo: `cargo run --release -p polytabletop-rpg`.
- Installati da `install.sh`: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, la pagina di manuale e l'alias breve `ptt` (`ptt list` = `polytabletop list`).
- Variabili d'ambiente: `POLYTABLETOP_LANG` (lingua), `POLYTABLETOP_LEVEL` (easy, normal, hard o expert), `POLYTABLETOP_AI_MS` (budget di tempo dell'IA, in millisecondi).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # sei il Giocatore 1 (il bianco)
./target/release/polytabletop play go --you two      # inizia il computer
./target/release/polytabletop --lang de play barbut --seed 42
```

## Giochi

Si possono giocare subito 240 giochi, nel terminale e nell'interfaccia grafica; il catalogo ha 1200 voci (`polytabletop catalog` termina con: implementate 239, giochi tradizionali previsti 214, varianti proprie previste 475, toolkit generico 272). Le 272 voci toolkit sono i 84 sistemi di gioco di ruolo (gli altri 16 sono avventure giocabili), serviti dal crate generico `polytabletop-rpg` (dadi, schede, prove di abilità, generatore di dungeon, simulatore di battaglie, diario), più i 93 titoli di miniature e i 95 di carte collezionabili non ancora nel programma, tutti senza le regole di alcun editore. Ognuna delle dodici categorie del catalogo ha esattamente 100 voci distinte. Categorie e numero di giochi, da `polytabletop-games/src/lib.rs`:

| Categoria | Giochi | Esempi (id) |
|---|---|---|
| Carte | 70 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda`, `pinochle`, `gin_rummy` |
| Astratti | 50 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane`, `oware`, `bao` |
| Strategia | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, più varianti originali di giochi moderni |
| Cooperativi | 22 | `artificii`, `focare`, `strajile`, `cripta`, `turnul_zorilor`, `arhiva_abisala` |
| Party | 17 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Dadi | 17 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut`, `sicbo`, `farkle` |
| Classici | 17 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo`, `riichi`, `senet` |
| Famiglia | 10 | `orasel`, `roata`, `turnul`, `zidurile`, `ashtachamma`, `crokinole` |
| Miniature | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Carte collezionabili (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Deduzione | 1 | `conac` |
| Parole | 1 | `litere` |
| **Totale** | **240** | |

475 voci del catalogo sono previste come varianti originali con titolo proprio (per esempio Twister, Campionii Distracției, Turistico) e 214 come giochi tradizionali con regole pubbliche. Le regole in breve di ogni gioco sono in [docs/JOCURI.md](docs/JOCURI.md), il catalogo generato in [docs/CATALOG.md](docs/CATALOG.md). Nell'interfaccia grafica 36 giochi hanno una scacchiera disegnata con mosse a clic (scacchi, dama, reversi, forza quattro, gomoku, go, mulino, xiangqi, shogi, backgammon, ludo e pachisi, hex, la famiglia mancala e altri); gli altri giochi compaiono come testo, con le mosse legali come pulsanti.

## Lingue

PolyTabletop parla 8 lingue: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. Le interfacce a riga di comando e grafica sono tradotte in tutte e 8. La lingua si sceglie con `--lang xx`, poi `POLYTABLETOP_LANG`, poi il `LANG` di sistema, altrimenti il rumeno. Titoli e regole di tutti i 240 giochi sono tradotti nelle 8 lingue (`polytabletop-games/data/i18n/`, con ripiego sul testo rumeno se manca una voce); il testo all'interno dei giochi (carte, banche di contenuti, messaggi) è ancora in rumeno. La notazione delle mosse (`e2e4`, `c3-d4`) non cambia mai. Dettagli: [docs/I18N.md](docs/I18N.md).

## IA

- I giochi astratti usano la ricerca alfa-beta con approfondimento iterativo, budget di tempo, tabella di trasposizione e quiescenza; i giochi di dadi, di carte e con grande fattore di ramificazione usano MCTS (UCT). Dettagli in [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- I livelli `easy`, `normal`, `hard`, `expert` scalano il budget di tempo e il numero di simulazioni (circa 0,1x, 1x, 2,5x e 6x); a `easy` l'IA gioca talvolta di proposito una mossa più debole.
- L'IA non imbroglia: valutazione e simulazioni leggono solo le informazioni visibili al giocatore per cui gioca; nei giochi con mani nascoste l'IA ipotizza a caso la parte ignota invece di guardare la mano dell'avversario.
- Informazione nascosta e hot-seat: in modalità `--you both`, i giochi elencati in `HOTSEAT_SAFE` mostrano solo l'informazione privata di chi deve muovere; per gli altri giochi con stato nascosto casuale compare un avviso e `hint` è disattivato. Ogni gioco appartiene a una di tre categorie (sicuro, un umano contro i robot con vista totale per un secondo umano, oppure senza informazione privata), verificata da un test. Vedi [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Gioco in rete

`polytabletop host ID` attende un client sulla porta 7878 (o `--port P`) e gioca come Giocatore 1 (oppure 2 o a caso con `--side`); `polytabletop join HOST:PORTA` si collega. Il protocollo testuale `polytabletop/1` è autorevole sul server: l'unica copia della partita sta lì, e ogni lato vede solo le proprie informazioni private. Chi non muove entro `--timeout` secondi (600 per impostazione predefinita) perde. Nel terminale si scrive una mossa, `?`, `state`, `chat TESTO` o `q`. Protocollo e dettagli: [docs/RETEA.md](docs/RETEA.md). Il gioco in rete è provato in loopback (`polytabletop/tests/net_loopback.rs`), non su reti reali.

## Pagine delle regole

Ogni gioco giocabile ha una pagina con scopo, preparazione, struttura del turno, sintassi delle mosse, regole speciali, punteggio, fine della partita, note sull'IA e differenze rispetto alle regole tradizionali: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (giochi da 1 a 120) e [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (giochi da 121 a 240). Le pagine sono in rumeno.

## Architettura

| Crate | Ruolo |
|---|---|
| `polytabletop-core` | trait tipizzato `Game`, trait `GameObj` utilizzabile come oggetto (mosse come testo, per qualsiasi interfaccia), IA (alfa-beta, MCTS), generatore casuale proprio, i18n; senza dipendenze esterne |
| `polytabletop-games` | tutti i giochi (un modulo ciascuno), `playable()`, `HOTSEAT_SAFE`, catalogo generato; senza dipendenze esterne |
| `polytabletop-rpg` | kit generico per giochi di ruolo; senza dipendenze esterne |
| `polytabletop-gui` | interfaccia grafica egui/eframe; l'unico crate con dipendenze esterne, fuori dalla compilazione predefinita |
| `polytabletop` | interfaccia a riga di comando, arena, rete, trascrizioni |

Un nuovo gioco è una struttura che implementa `Game` (mosse legali, applicazione, esito, valutazione, visualizzazione) più una voce in `playable()`; ogni interfaccia lo riceve gratis. Vedi [docs/ARHITECTURA.md](docs/ARHITECTURA.md) e [docs/GUI.md](docs/GUI.md) (documenti in rumeno).

## Compilazione, test e CI

- Compilazione: `cargo build --release` (Rust, toolchain stabile recente). L'interfaccia grafica non fa parte della compilazione predefinita: `cargo build --release -p polytabletop-gui`; su Linux richiede le librerie di sviluppo X11 o Wayland.
- Test: `cargo test --release` (perft agli scacchi 20 / 400 / 8.902 / 197.281, regole speciali, partite casuali complete per ogni gioco, IA contro mosse casuali, audit del punto di vista, coerenza del catalogo, rete in loopback).
- Prima di una pull request: `cargo fmt --all` e `cargo clippy --release --all-targets -- -D warnings`.
- La CI (`.github/workflows/ci.yml`) esegue fmt, clippy e i test su Ubuntu 26.04, poi compila l'interfaccia grafica ed esegue `polytabletop-gui --selftest`. Un tag `v*` avvia `release.yml`, che costruisce archivi per Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) con somme SHA-256; `install.sh` li installa da `chiuta/PolyTabletop`. È incluso anche un `Dockerfile` (solo CLI e RPG).

## Clean-room e marchi

- Le regole provengono solo da descrizioni pubbliche dei giochi; non si usano codice, testi, grafica o dati di prodotti commerciali, e le fonti pubbliche sono annotate nell'intestazione di ogni file di gioco.
- La meccanica di un gioco è libera, il nome, la grafica e i testi no: i giochi hanno nomi generici («Reversi», «Forza quattro») oppure, per i giochi moderni con marchio, varianti originali con titoli, temi e contenuti propri.
- Nessun marchio negli id, nei titoli, nei nomi delle carte o nelle banche di contenuti. I nomi di marchi compaiono solo nel catalogo, come obiettivi di pianificazione.
- I giochi di ruolo, con miniature e di carte collezionabili sono kit generici (dadi, schede, mazzi originali), mai una copia delle regole o dei contenuti di un editore.

## Limiti

- L'interfaccia grafica è stata provata solo senza schermo (`polytabletop-gui --selftest`); non è stata verificata visivamente su uno schermo reale.
- La forza dell'IA è stata misurata contro mosse casuali e tra budget di tempo, non contro giocatori umani; alcuni giochi hanno limiti noti (per esempio la percentuale di vittorie di `macao` e `hruba` è riportata nel changelog).
- Le regole dei giochi scritte a memoria sono state confrontate con fonti pubbliche, non con ogni regolamento ufficiale, e i giochi non sono stati provati da giocatori esterni.
- Il gioco in rete è provato solo in loopback.
- Titoli e regole dei giochi sono tradotti, ma il testo all'interno dei giochi (carte, banche di contenuti, messaggi) è ancora in rumeno; le traduzioni non sono state riviste da madrelingua.
- Le 16 voci di giochi di ruolo sono servite da un kit generico, non da implementazioni di quei sistemi.

## Contribuire e cronologia

I contributi sono benvenuti nel rispetto delle regole clean-room: vedi [CONTRIBUTING.md](CONTRIBUTING.md) (come aggiungere un gioco, test, lista di controllo per le pull request). Cronologia delle versioni: [CHANGELOG.md](CHANGELOG.md).

## Licenza

Dichiarata in `Cargo.toml` come `MIT OR Apache-2.0`; testi completi in [LICENSE-MIT](LICENSE-MIT) e [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Crediti

Autore: Alexio (Alexandru-Ionuț Chiuță), nella serie «Fără Frontiere» (Senza Frontiere) di applicazioni trade-free. Contatto: alexio@trom.tf. Pagina di presentazione: <https://polytabletop.chiuta.ro>.
