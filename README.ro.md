[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 0.9.0 · Last updated: 2026-11-26

## Ce este

PolyTabletop (fostul MASA) este o colecție clean-room de jocuri de masă scrisă în Rust: 171 de jocuri jucabile contra calculatorului, în terminal sau în fereastră grafică, plus modul „la masă” (hot-seat) și joc în rețea pentru doi oameni. Clasice abstracte, jocuri de cărți și de zaruri, variante originale ale mecanicilor moderne cu nume proprii, jocuri cooperative cu partener AI, jocuri cu roluri ascunse în care roboții completează masa, jocuri de petrecere și de cuvinte, motoare de miniaturi și de cărți colecționabile cu cărți originale. Motorul și jocurile nu au dependențe externe; doar interfața grafică (`polytabletop-gui`) folosește `eframe`/egui. Catalogul urmărește 201 de jocuri-țintă.

Pagina de prezentare: <https://polytabletop.chiuta.ro>. Ghid scurt de instalare: [docs/INSTALL.ro.md](docs/INSTALL.ro.md) ([English](docs/INSTALL.en.md) · [Français](docs/INSTALL.fr.md) · [Italiano](docs/INSTALL.it.md) · [Español](docs/INSTALL.es.md) · [Português](docs/INSTALL.pt.md) · [Deutsch](docs/INSTALL.de.md) · [Magyar](docs/INSTALL.hu.md)).

## Folosire

- Listă și catalog: `polytabletop list` arată cele 171 de jocuri jucabile cu regulile pe scurt; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` arată starea celor 201 de intrări din catalog.
- Joc: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (implicit) te face Jucătorul 1, `two` Jucătorul 2, `both` este „la masă”, la o singură tastatură, iar `none` lasă calculatorul să joace ambele părți. `--seed N` face reproductibile împărțirea cărților și zarurile; `--record FIȘIER` salvează o transcriere.
- În joc: scrii mutarea exact cum o listează `?` (de exemplu `e2e4`, `c3xe5`, `d6`), `?` = mutări legale, `hint` = sugestia AI-ului, `q` = ieșire.
- AI contra AI: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (vezi [docs/ARENA.md](docs/ARENA.md)).
- Doi oameni în rețea: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` și `polytabletop join HOST:PORT`.
- Transcrieri: `polytabletop replay FIȘIER [--step]`, `polytabletop export FIȘIER --format pgn|text|json` (PGN doar pentru șah), `polytabletop import-pgn FIȘIER [--out TRANSCRIERE]`.
- Limbi: `polytabletop langs` listează cele 8 limbi ale interfeței; `--lang xx` poate sta oriunde în linia de comandă.
- Interfața grafică: `cargo run --release -p polytabletop-gui` (`-- --selftest` rulează verificările fără ecran). Trusa pentru jocuri de rol: `cargo run --release -p polytabletop-rpg`.
- Instalate de `install.sh`: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, pagina de manual și aliasul scurt `ptt` (`ptt list` = `polytabletop list`).
- Variabile de mediu: `POLYTABLETOP_LANG` (limba), `POLYTABLETOP_LEVEL` (easy, normal, hard sau expert), `POLYTABLETOP_AI_MS` (bugetul de timp al AI-ului, în milisecunde).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # tu ești Jucătorul 1 (albul)
./target/release/polytabletop play go --you two      # calculatorul începe
./target/release/polytabletop --lang de play barbut --seed 42
```

## Jocuri

Se pot juca acum 171 de jocuri, în terminal și în interfața grafică; catalogul are 201 de intrări (`polytabletop catalog` se încheie cu: implementate 182, variante proprii planificate 3, toolkit generic 16). Cele 16 intrări toolkit sunt sisteme de jocuri de rol, deservite de crate-ul generic `polytabletop-rpg` (zaruri, fișe, teste de abilitate, generator de dungeon, simulator de bătălii, jurnal), fără regulile vreunui editor. Categoriile și numărul de jocuri, din `polytabletop-games/src/lib.rs`:

| Categorie | Jocuri | Exemple (id-uri) |
|---|---|---|
| Cărți | 45 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda` |
| Abstract | 40 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane` |
| Strategie | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, plus variante originale ale unor jocuri moderne |
| Party | 16 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Zaruri | 11 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut` |
| Clasic | 9 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo` |
| Familie | 7 | `orasel`, `roata`, `turnul`, `zidurile` |
| Miniaturi | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Cooperativ | 6 | `artificii`, `focare`, `strajile`, `cripta` |
| Cărți colecționabile (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Deducție | 1 | `conac` |
| Cuvinte | 1 | `litere` |
| **Total** | **171** | |

Trei intrări din catalog rămân planificate ca variante originale (Twister, Campionii Distracției, Turistico). Regulile pe scurt ale fiecărui joc sunt în [docs/JOCURI.md](docs/JOCURI.md), catalogul generat în [docs/CATALOG.md](docs/CATALOG.md). În interfața grafică, 36 de jocuri au tablă desenată cu mutări prin click (șah, dame, reversi, patru în linie, gomoku, go, moara, xiangqi, shogi, table, ludo și pachisi, hex, familia mancala și altele); celelalte jocuri apar în text, cu mutările legale ca butoane.

## Limbi

PolyTabletop vorbește 8 limbi: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. Interfețele în linie de comandă și grafică sunt traduse în toate 8. Limba se alege prin `--lang xx`, apoi `POLYTABLETOP_LANG`, apoi `LANG` din sistem, altfel română. Titlurile și regulile tuturor celor 171 de jocuri sunt traduse în cele 8 limbi (`polytabletop-games/data/i18n/`, cu revenire la textul românesc dacă lipsește o intrare); textul din interiorul jocurilor (cărți, bănci de conținut, replici) este încă în română. Notația mutărilor (`e2e4`, `c3-d4`) nu se schimbă niciodată. Detalii: [docs/I18N.md](docs/I18N.md).

## AI

- Jocurile abstracte folosesc căutare alpha-beta cu adâncire iterativă, buget de timp, tabel de transpoziție și quiescence; jocurile cu zaruri, cărți și factor de ramificare mare folosesc MCTS (UCT). Detalii în [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Nivelurile `easy`, `normal`, `hard`, `expert` scalează bugetul de timp și numărul de simulări (aproximativ 0,1x, 1x, 2,5x și 6x); la `easy` AI-ul joacă uneori intenționat o mutare mai slabă.
- AI-ul nu trișează: evaluarea și simulările lui citesc doar informația vizibilă jucătorului pentru care joacă; jocurile cu mâini ascunse ghicesc la întâmplare partea necunoscută, în loc să se uite în mâna adversarului.
- Informație ascunsă și „la masă”: în modul `--you both`, jocurile din `HOTSEAT_SAFE` arată doar informația privată a celui la mutare; pentru celelalte jocuri cu stare ascunsă aleatoare apare un avertisment, iar `hint` este dezactivat. Fiecare joc face parte din una de trei categorii (sigur, un om contra roboților cu vedere totală pentru un al doilea om, sau fără informație privată), verificat de un test. Vezi [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Joc în rețea

`polytabletop host ID` așteaptă un client pe portul 7878 (sau `--port P`) și joacă el ca Jucătorul 1 (sau 2 ori aleatoriu cu `--side`); `polytabletop join HOST:PORT` se conectează. Protocolul text `polytabletop/1` este autoritar pe server: singura copie a jocului e acolo, iar fiecare parte vede doar informația ei privată. Cine nu mută în `--timeout` secunde (implicit 600) pierde. În terminal scrii o mutare, `?`, `state`, `chat TEXT` sau `q`. Protocolul și detaliile: [docs/RETEA.md](docs/RETEA.md). Jocul în rețea este testat prin bucla locală (`polytabletop/tests/net_loopback.rs`), nu în rețele reale.

## Pagini de reguli

Fiecare joc jucabil are o pagină cu scopul, pregătirea, structura turei, sintaxa mutărilor, regulile speciale, punctajul, sfârșitul jocului, note despre AI și diferențele față de regulile tradiționale: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (jocurile 1–86) și [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (jocurile 87–171). Paginile sunt în română.

## Arhitectură

| Crate | Rol |
|---|---|
| `polytabletop-core` | trait-ul tipat `Game`, trait-ul obiect-sigur `GameObj` (mutări ca text, pentru orice interfață), AI (alpha-beta, MCTS), generator aleator propriu, i18n; fără dependențe externe |
| `polytabletop-games` | toate jocurile (un modul pe joc), `playable()`, `HOTSEAT_SAFE`, catalogul generat; fără dependențe externe |
| `polytabletop-rpg` | trusă generică pentru jocuri de rol; fără dependențe externe |
| `polytabletop-gui` | interfața grafică egui/eframe; singurul crate cu dependențe externe, în afara build-ului implicit |
| `polytabletop` | interfața în linie de comandă, arena, rețeaua, transcrierile |

Un joc nou este o structură care implementează `Game` (mutări legale, aplicare, rezultat, evaluare, afișare) plus o intrare în `playable()`; orice front-end îl primește gratis. Vezi [docs/ARHITECTURA.md](docs/ARHITECTURA.md) și [docs/GUI.md](docs/GUI.md).

## Compilare, teste și CI

- Compilare: `cargo build --release` (Rust, lanț de unelte stabil recent). Interfața grafică nu face parte din build-ul implicit: `cargo build --release -p polytabletop-gui`; pe Linux are nevoie de bibliotecile de dezvoltare X11 sau Wayland.
- Teste: `cargo test --release` (perft la șah 20 / 400 / 8 902 / 197 281, reguli speciale, partide aleatorii complete pe fiecare joc, AI contra mutări aleatorii, auditul privitorului, coerența catalogului, rețeaua prin bucla locală).
- Înainte de un pull request: `cargo fmt --all` și `cargo clippy --release --all-targets -- -D warnings`.
- CI (`.github/workflows/ci.yml`) rulează fmt, clippy și testele pe Ubuntu 26.04, apoi compilează interfața grafică și rulează `polytabletop-gui --selftest`. O etichetă `v*` declanșează `release.yml`, care construiește arhive pentru Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) cu sume SHA-256; `install.sh` le instalează din `chiuta/PolyTabletop`. Este inclus și un `Dockerfile` (doar CLI și RPG).

## Clean-room și mărci

- Regulile vin doar din descrieri publice ale jocurilor; nu se folosește cod, text, grafică sau date ale produselor comerciale, iar sursele publice sunt notate în antetul fiecărui fișier de joc.
- Mecanica unui joc este liberă, numele, grafica și textele nu: jocurile poartă nume generice („Reversi”, „Patru în linie”) sau, pentru jocurile moderne cu marcă, variante originale cu titluri, teme și conținut proprii.
- Fără nume de mărci în id-uri, titluri, nume de cărți sau bănci de conținut. Numele de mărci apar doar în catalog, ca ținte de planificare.
- Jocurile de rol, cu miniaturi și cărțile colecționabile sunt truse generice (zaruri, fișe, pachete originale), nu copii ale regulilor sau conținutului unui editor.

## Limite

- Interfața grafică a fost testată doar fără ecran (`polytabletop-gui --selftest`); nu a fost verificată vizual pe un ecran real.
- Forța AI-ului a fost măsurată contra mutărilor aleatorii și între bugete de timp, nu contra jucătorilor umani; unele jocuri au limite cunoscute (de exemplu rata de câștig la `macao` și `hruba` este consemnată în changelog).
- Regulile jocurilor scrise din memorie au fost confruntate cu surse publice, nu cu fiecare regulament oficial, iar jocurile nu au fost testate de jucători din afară.
- Jocul în rețea este testat doar prin bucla locală.
- Titlurile și regulile jocurilor sunt traduse, dar textul din interiorul jocurilor (cărți, bănci de conținut, replici) este încă în română; traducerile nu au fost revizuite de vorbitori nativi.
- Cele 16 intrări de jocuri de rol sunt deservite de o trusă generică, nu de implementări ale acelor sisteme.

## Contribuții și istoric

Contribuțiile sunt binevenite în regulile clean-room: vezi [CONTRIBUTING.md](CONTRIBUTING.md) (cum adaugi un joc, teste, lista de verificare pentru pull request). Istoricul versiunilor: [CHANGELOG.md](CHANGELOG.md).

## Licență

Declarată în `Cargo.toml` ca `MIT OR Apache-2.0`. Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Credite

Autor: Alexio (Alexandru-Ionuț Chiuță), parte din seria „Fără Frontiere” de aplicații trade-free. Contact: alexio@trom.tf. Pagina de prezentare: <https://polytabletop.chiuta.ro>.
