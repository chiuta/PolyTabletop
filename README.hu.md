[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

# **PolyTabletop**

Version: 1.0.0 · Last updated: 2026-11-28

## Mi ez?

A PolyTabletop Rustban írt, clean-room táblás és asztali játékgyűjtemény: 1200 játék játszható a számítógép ellen, terminálban vagy grafikus ablakban, emellett van hot-seat mód egy gépen és hálózati játék két embernek. Vannak absztrakt klasszikusok, kártya- és kockajátékok, modern mechanizmusok eredeti, saját nevű változatai, kooperatív játékok MI-társsal, rejtett szerepű játékok, ahol robotok töltik be az asztal többi helyét, társasági és szójátékok, valamint figura- és gyűjthető kártyás játékmotorok eredeti kártyákkal. A motornak és a játékoknak nincs külső függősége; csak a grafikus felület (`polytabletop-gui`) használja az `eframe`/egui könyvtárat. A katalógus 1200 célként kitűzött játékot tart nyilván.

Bemutatóoldal: <https://polytabletop.chiuta.ro>. Rövid telepítési útmutató: [docs/INSTALL.hu.md](docs/INSTALL.hu.md) ([English](docs/INSTALL.en.md) · [Română](docs/INSTALL.ro.md) · [Français](docs/INSTALL.fr.md) · [Italiano](docs/INSTALL.it.md) · [Español](docs/INSTALL.es.md) · [Português](docs/INSTALL.pt.md) · [Deutsch](docs/INSTALL.de.md)).

## Használat

- Lista és katalógus: a `polytabletop list` kiírja a 1200 játszható játékot röviden a szabályaikkal; a `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` a katalógus mind a 1200 bejegyzésének állapotát mutatja.
- Játék: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. A `--you one` (alapértelmezett) az 1. játékossá tesz, a `two` a 2.-á, a `both` hot-seat mód egyetlen billentyűzeten, a `none` pedig a gépre bízza mindkét oldalt. A `--seed N` reprodukálhatóvá teszi a laposztást és a kockadobásokat; a `--record FÁJL` jegyzőkönyvet ment.
- Játék közben: a lépést pontosan úgy írd be, ahogy a `?` kilistázza (például `e2e4`, `c3xe5`, `d6`), `?` = szabályos lépések, `hint` = az MI javaslata, `q` = kilépés.
- MI az MI ellen: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (lásd [docs/ARENA.md](docs/ARENA.md)).
- Két ember hálózaton: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` és `polytabletop join GÉP:PORT`.
- Jegyzőkönyvek: `polytabletop replay FÁJL [--step]`, `polytabletop export FÁJL --format pgn|text|json` (a PGN csak sakkhoz), `polytabletop import-pgn FÁJL [--out JEGYZŐKÖNYV]`.
- Nyelvek: a `polytabletop langs` kiírja a felület 8 nyelvét; a `--lang xx` a parancssor bármely helyén állhat.
- Grafikus felület: `cargo run --release -p polytabletop-gui` (a `-- --selftest` képernyő nélküli ellenőrzéseket futtat). Szerepjátékos eszköztár: `cargo run --release -p polytabletop-rpg`.
- Az `install.sh` telepíti: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, a kézikönyvoldalt és a rövid `ptt` aliast (`ptt list` = `polytabletop list`).
- Környezeti változók: `POLYTABLETOP_LANG` (nyelv), `POLYTABLETOP_LEVEL` (easy, normal, hard vagy expert), `POLYTABLETOP_AI_MS` (az MI időkerete ezredmásodpercben).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # te vagy az 1. játékos (világos)
./target/release/polytabletop play go --you two      # a gép kezd
./target/release/polytabletop --lang de play barbut --seed 42
```

## Játékok

Jelenleg 1200 játék játszható, terminálban és a grafikus felületen is; a katalógusnak 1200 bejegyzése van (a `polytabletop catalog` kimenete így zárul: megvalósítva 239, tervezett hagyományos játék 214, tervezett saját változat 475, általános eszköztár 272). A 272 eszköztár-bejegyzés a 84 szerepjátékrendszer (a másik 16 játszható kaland), amelyeket az általános `polytabletop-rpg` crate szolgál ki (kockák, karakterlapok, képességpróbák, kazamata-generátor, csataszimulátor, napló), valamint a programban még nem szereplő 93 miniatűr- és 95 gyűjthetőkártya-cím, mind bármely kiadó szabályai nélkül. A katalógus tizenkét kategóriájának mindegyike pontosan 100 különböző bejegyzést tartalmaz. A kategóriák és a játékok száma a `polytabletop-games/src/lib.rs` alapján:

| Kategória | Játékok | Példák (azonosítók) |
|---|---|---|
| Absztrakt | 100 | |
| Klasszikus | 100 | |
| Család | 100 | |
| Stratégia | 100 | |
| Társas/party | 100 | |
| Kooperatív | 100 | |
| Kártya | 100 | |
| Szerepjáték | 100 | |
| Miniatúrák | 100 | |
| Gyűjthető kártyák | 100 | |
| Gyerekek | 100 | |
| Kocka | 100 | |
| **Összesen** | **1200** | |

A katalógus 475 bejegyzése saját címmel ellátott eredeti változatként van tervezve (például Twister, Campionii Distracției, Turistico), 214 pedig nyilvános szabályú hagyományos játékként. Minden játék rövid szabályai a [docs/JOCURI.md](docs/JOCURI.md) fájlban, a generált katalógus a [docs/CATALOG.md](docs/CATALOG.md) fájlban található. A grafikus felületen 36 játéknak van megrajzolt táblája kattintással tehető lépésekkel (sakk, dáma, reversi, négy egy sorban, gomoku, go, malom, xiangqi, shogi, backgammon, ludo és pachisi, hex, a mancala család és továbbiak); a többi játék szövegként jelenik meg, a szabályos lépések gombokként.

## Nyelvek

A PolyTabletop 8 nyelven szól: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. A parancssori és a grafikus felület mind a 8 nyelvre le van fordítva. A nyelvet a `--lang xx` választja ki, utána a `POLYTABLETOP_LANG`, majd a rendszer `LANG` értéke, különben a román. Mind a 1200 játék címe és szabálya le van fordítva a 8 nyelvre (`polytabletop-games/data/i18n/`, román tartalék szöveggel, ha egy bejegyzés hiányzik); a játékokon belüli szöveg (kártyák, tartalombankok, üzenetek) még román. A lépésjelölés (`e2e4`, `c3-d4`) soha nem változik. Részletek: [docs/I18N.md](docs/I18N.md).

## Mesterséges intelligencia

- Az absztrakt játékok alfa-béta keresést használnak iteratív mélyítéssel, időkerettel, transzpozíciós táblával és nyugalmi kereséssel; a kocka-, kártyajátékok és a nagy elágazási tényezőjű játékok MCTS-t (UCT) használnak. Részletek: [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Az `easy`, `normal`, `hard`, `expert` szint az időkeretet és a szimulációk számát skálázza (nagyjából 0,1x, 1x, 2,5x és 6x); `easy` szinten az MI néha szándékosan gyengébb lépést tesz.
- Az MI nem csal: az értékelése és a szimulációi csak azt az információt olvassák, amely annak a játékosnak látható, akiért játszik; a rejtett kezű játékokban az ismeretlen részt véletlenszerűen találja ki, ahelyett hogy az ellenfél kezébe nézne.
- Rejtett információ és hot-seat: `--you both` módban a `HOTSEAT_SAFE` listán szereplő játékok csak a soron következő játékos privát információját mutatják; a véletlenszerű rejtett állapotú többi játéknál figyelmeztetés jelenik meg, és a `hint` ki van kapcsolva. Minden játék a három osztály egyikébe tartozik (biztonságos, egy ember robotok ellen teljes nézettel egy második embernek, vagy privát információ nélküli), amit egy teszt ellenőriz. Lásd [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Hálózati játék

A `polytabletop host ID` a 7878-as porton (vagy a `--port P` szerinti porton) vár egy kliensre, és maga az 1. játékosként játszik (a `--side` szerint 2.-ként vagy véletlenszerűen); a `polytabletop join GÉP:PORT` csatlakozik. A `polytabletop/1` szöveges protokoll a szerveren az irányadó: a játszma egyetlen példánya ott él, és mindkét oldal csak a saját privát információját látja. Aki `--timeout` másodpercen belül (alapértelmezés: 600) nem lép, veszít. Terminálban lépést, `?`, `state`, `chat SZÖVEG` vagy `q` parancsot lehet beírni. Protokoll és részletek: [docs/RETEA.md](docs/RETEA.md). A hálózati játékot loopbacken teszteltük (`polytabletop/tests/net_loopback.rs`), valódi hálózaton nem.

## Szabályoldalak

Minden játszható játéknak van oldala a céllal, az előkészítéssel, a kör felépítésével, a lépések szintaxisával, a különleges szabályokkal, a pontozással, a játék végével, az MI-re vonatkozó megjegyzésekkel és a hagyományos szabályoktól való eltérésekkel: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (1–120. játék) és [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (121–240. játék). Az oldalak románul vannak.

## Felépítés

| Crate | Szerep |
|---|---|
| `polytabletop-core` | típusos `Game` trait, objektumbiztos `GameObj` trait (lépések szövegként, bármely felülethez), MI (alfa-béta, MCTS), saját véletlenszám-generátor, i18n; külső függőség nélkül |
| `polytabletop-games` | az összes játék (egy-egy modul), `playable()`, `HOTSEAT_SAFE`, generált katalógus; külső függőség nélkül |
| `polytabletop-rpg` | általános eszköztár szerepjátékokhoz; külső függőség nélkül |
| `polytabletop-gui` | egui/eframe grafikus felület; az egyetlen külső függőségű crate, az alapértelmezett fordításon kívül |
| `polytabletop` | parancssori felület, aréna, hálózat, jegyzőkönyvek |

Egy új játék egy `Game`-et megvalósító struktúra (szabályos lépések, alkalmazás, eredmény, értékelés, megjelenítés) plusz egy bejegyzés a `playable()` függvényben; minden felület ingyen megkapja. Lásd [docs/ARHITECTURA.md](docs/ARHITECTURA.md) és [docs/GUI.md](docs/GUI.md) (román nyelvű dokumentumok).

## Fordítás, tesztelés és CI

- Fordítás: `cargo build --release` (Rust, friss stabil eszközlánc). A grafikus felület nem tartozik az alapértelmezett fordításhoz: `cargo build --release -p polytabletop-gui`; Linuxon az X11 vagy a Wayland fejlesztői könyvtárai kellenek hozzá.
- Tesztelés: `cargo test --release` (sakk-perft 20 / 400 / 8 902 / 197 281, különleges szabályok, minden játékra teljes véletlen játszmák, MI a véletlen lépések ellen, nézőpont-ellenőrzés, katalóguskövetkezetesség, hálózat loopbacken).
- Pull request előtt: `cargo fmt --all` és `cargo clippy --release --all-targets -- -D warnings`.
- A CI (`.github/workflows/ci.yml`) Ubuntu 26.04-en futtatja a fmt-t, a clippyt és a teszteket, majd lefordítja a grafikus felületet és lefuttatja a `polytabletop-gui --selftest` parancsot. Egy `v*` címke elindítja a `release.yml`-t, amely Linuxra (x86_64, aarch64), Windowsra és macOS-re (arm64, x86_64) épít archívumokat SHA-256 összegekkel; az `install.sh` ezeket telepíti a `chiuta/PolyTabletop` tárolóból. Mellékelve van egy `Dockerfile` is (csak CLI és RPG).

## Clean-room és védjegyek

- A szabályok kizárólag a játékok nyilvános leírásaiból származnak; kereskedelmi termékek kódját, szövegét, grafikáját vagy adatait nem használjuk, a nyilvános forrásokat pedig minden játékfájl fejlécében feltüntetjük.
- A játék mechanikája szabad, a neve, a grafikája és a szövege nem: a játékok általános nevet viselnek („Reversi”, „Négy egy sorban”), a modern márkás játékoknál pedig saját címmel, témával és tartalommal rendelkező eredeti változatok.
- Nincsenek védjegyek az azonosítókban, címekben, kártyanevekben és tartalombankokban. A márkanevek csak a katalógusban szerepelnek, tervezési célokként.
- A szerepjátékok, figurás és gyűjthető kártyás játékok általános eszköztárak (kockák, lapok, eredeti paklik), soha nem egy kiadó szabályainak vagy tartalmának másolatai.

## Korlátok

- A grafikus felületet csak képernyő nélkül teszteltük (`polytabletop-gui --selftest`); valódi képernyőn vizuálisan nem ellenőriztük.
- Az MI erejét véletlen lépések ellen és időkeretek között mértük, emberi játékosok ellen nem; egyes játékoknak ismert korlátaik vannak (például a `macao` és a `hruba` győzelmi aránya a changelogban szerepel).
- Az emlékezetből leírt játékszabályokat nyilvános forrásokkal vetettük össze, nem minden hivatalos szabálykönyvvel, és a játékokat külső játékosok nem tesztelték.
- A hálózati játékot csak loopbacken teszteltük.
- A játékcímek és szabályok le vannak fordítva, de a játékokon belüli szöveg (kártyák, tartalombankok, üzenetek) még román; a fordításokat anyanyelvi beszélők nem lektorálták.
- A 16 szerepjáték-bejegyzést általános eszköztár szolgálja ki, nem ezeknek a rendszereknek a megvalósításai.

## Közreműködés és változásnapló

A közreműködést szívesen fogadjuk a clean-room szabályok keretében: lásd [CONTRIBUTING.md](CONTRIBUTING.md) (játék hozzáadása, tesztek, pull request ellenőrzőlista). Verziótörténet: [CHANGELOG.md](CHANGELOG.md).

## Licenc

A `Cargo.toml`-ban `MIT OR Apache-2.0` néven deklarálva; a teljes szövegek: [LICENSE-MIT](LICENSE-MIT) és [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Készítők

Szerző: Alexio (Alexandru-Ionuț Chiuță), a „Fără Frontiere” (Határok nélkül) trade-free alkalmazássorozat része. Kapcsolat: alexio@trom.tf. Bemutatóoldal: <https://polytabletop.chiuta.ro>.
