# Telepítés és gyors kezdés

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## Mi ez?

**PolyTabletop** társasjátékok gyűjteménye (sakk, go, dáma, kártya- és kockajátékok és még sok más), amelyet a nulláról írtunk Rustban; játszható terminálban, grafikus felületen és helyi hálózaton. A román szöveg a mérvadó; ez az oldal egy rövid útmutató.

## Telepítés

- **Kiadási archívumból.** Töltsd le a platformodnak megfelelő archívumot a projekt Releases oldaláról, csomagold ki bárhová (nincs szükség telepítőre vagy külső könyvtárakra), és futtasd a `polytabletop` programot abból a mappából.
- **A telepítőszkripttel (Linux és macOS).** A `bash install.sh [verziócímke]` (a `chiuta/PolyTabletop` tárolóban lévő szkript) letölti a platformodnak megfelelő legújabb kiadási archívumot, ellenőrzi az SHA-256 összeget, és telepíti a `polytabletop`, `polytabletop-gui` és `polytabletop-rpg` programot a `~/.local/bin` mappába (a `PREFIX=...` változóval módosítható), valamint a kézikönyvoldalt és a rövid `ptt` aliast. Windowson töltsd le a `.zip` archívumot a Releases oldalról.
- **Forráskódból, cargóval** (Rust, friss stabil eszközlánc):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Grafikus felület (nem kötelező): `cargo run --release -p polytabletop-gui`.

## Az első játék

A `polytabletop list` kilistázza a játékokat; a `polytabletop play <játék>` elindít egyet. Írd be a lépést (pl. `e2e4`), `?` = szabályos lépések, `hint` = az MI javaslata, `q` = kilépés.

## Egy gépen ketten vagy az MI ellen

```
./target/release/polytabletop play chess --you one     # az 1. játékost játszod (alapértelmezett)
./target/release/polytabletop play chess --you two     # a 2. játékost játszod (a számítógép kezd)
./target/release/polytabletop play chess --you both    # egy gépen ketten: mindkét játékos ugyanannál a billentyűzetnél ül
./target/release/polytabletop play go --you two
```

## Reprodukálható játszmák (`--seed`)

A `--seed N` reprodukálhatóvá teszi a kártyaosztást és a kockadobásokat: ugyanaz a szám ugyanazt a játszmát adja.

```
./target/release/polytabletop play barbut --seed 42
```

## Helyi hálózat: házigazda és csatlakozás

Helyi hálózaton az egyik gép létrehozza a játszmát, a másik csatlakozik hozzá. A két gépnek látnia kell egymást (a tűzfal nyitva a választott porton).

```
./target/release/polytabletop host chess               # az egyik játékos létrehozza a játszmát
./target/release/polytabletop join 192.168.1.20:7878   # a másik a házigazda címével és portjával csatlakozik
```

## Nyelvek (`--lang`)

A felület 8 nyelven szól: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (sorrend: `--lang`, majd `POLYTABLETOP_LANG`, majd a rendszer `LANG` értéke; egyébként `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

További információ: [README](../README.hu.md).
