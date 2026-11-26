# Instalare și pornire rapidă

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## Ce este

**PolyTabletop** este o colecție de jocuri de masă (șah, go, dame, jocuri de cărți, de zaruri și multe altele) scrise de la zero în Rust, care se joacă în terminal, în interfață grafică și în rețea locală. Textul în română este cel de referință; această pagină este un ghid scurt.

## Instalare

- **Din arhiva de lansare.** Descarcă arhiva pentru platforma ta de pe pagina Releases a proiectului, dezarhiveaz-o oriunde (nu are nevoie de instalator sau biblioteci externe) și rulează executabilul `polytabletop` din acel dosar.
- **Cu scriptul de instalare (Linux și macOS).** `bash install.sh [etichetă-versiune]` (scriptul din depozitul `chiuta/PolyTabletop`) descarcă ultima arhivă de lansare pentru platforma ta, verifică suma SHA-256 și instalează `polytabletop`, `polytabletop-gui` și `polytabletop-rpg` în `~/.local/bin` (se schimbă cu `PREFIX=...`), plus pagina de manual și aliasul scurt `ptt`. Pe Windows descarcă arhiva `.zip` de pe Releases.
- **Din sursă, cu cargo** (Rust, lanț de unelte stabil recent):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Interfață grafică (opțional): `cargo run --release -p polytabletop-gui`.

## Primul joc

`polytabletop list` arată jocurile; `polytabletop play <joc>` pornește unul. Scrii mutarea (ex. `e2e4`), `?` = mutări legale, `hint` = sugestia AI-ului, `q` = ieșire.

## La masă (hot-seat) sau contra AI-ului

```
./target/release/polytabletop play chess --you one     # joci Jucătorul 1 (implicit)
./target/release/polytabletop play chess --you two     # joci Jucătorul 2 (calculatorul începe)
./target/release/polytabletop play chess --you both    # la masă: ambii jucători la aceeași tastatură
./target/release/polytabletop play go --you two
```

## Partide reproductibile (`--seed`)

`--seed N` face reproductibile împărțirea cărților și zarurile: același număr dă aceeași partidă.

```
./target/release/polytabletop play barbut --seed 42
```

## Rețea locală: găzduire și alăturare

În rețea locală, un calculator găzduiește partida, iar celălalt se conectează la el. Cele două calculatoare trebuie să se vadă între ele (firewall deschis pe portul ales).

```
./target/release/polytabletop host chess               # un jucător găzduiește partida
./target/release/polytabletop join 192.168.1.20:7878   # celălalt se alătură cu adresa și portul gazdei
```

## Limbi (`--lang`)

Interfața vorbește 8 limbi: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (alese prin `--lang`, apoi `POLYTABLETOP_LANG`, apoi `LANG` din sistem; altfel `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

Mai multe: [README](../README.ro.md).
