# Instalación e inicio rápido

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## Qué es

**PolyTabletop** es una colección de juegos de mesa (ajedrez, go, damas, juegos de cartas, de dados y muchos más) escritos desde cero en Rust, que se pueden jugar en la terminal, en una interfaz gráfica y en red local. El texto en rumano es el de referencia; esta página es una guía breve.

## Instalación

- **Desde un archivo de versión.** Descarga el archivo para tu plataforma desde la página Releases del proyecto, descomprímelo donde quieras (no necesita instalador ni bibliotecas externas) y ejecuta el ejecutable `polytabletop` desde esa carpeta.
- **Con el script de instalación (Linux y macOS).** `bash install.sh [etiqueta-de-versión]` (script del repositorio `chiuta/PolyTabletop`) descarga el último archivo de versión para tu plataforma, comprueba su suma SHA-256 e instala `polytabletop`, `polytabletop-gui` y `polytabletop-rpg` en `~/.local/bin` (se cambia con `PREFIX=...`), además de la página de manual y el alias corto `ptt`. En Windows, descarga el archivo `.zip` desde Releases.
- **Desde el código fuente con cargo** (Rust, cadena de herramientas estable reciente):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Interfaz gráfica (opcional): `cargo run --release -p polytabletop-gui`.

## Primera partida

`polytabletop list` muestra los juegos; `polytabletop play <juego>` inicia uno. Escribe la jugada (p. ej. `e2e4`), `?` = jugadas legales, `hint` = sugerencia de la IA, `q` = salir.

## En el mismo equipo o contra la IA

```
./target/release/polytabletop play chess --you one     # juegas como Jugador 1 (predeterminado)
./target/release/polytabletop play chess --you two     # juegas como Jugador 2 (empieza el ordenador)
./target/release/polytabletop play chess --you both    # por turnos en el mismo equipo: ambos jugadores ante el mismo teclado
./target/release/polytabletop play go --you two
```

## Partidas reproducibles (`--seed`)

`--seed N` hace reproducibles el reparto de cartas y los dados: el mismo número da la misma partida.

```
./target/release/polytabletop play barbut --seed 42
```

## Red local: alojar y unirse

En red local, un equipo aloja la partida y el otro se conecta a él. Los dos equipos deben poder verse entre sí (cortafuegos abierto en el puerto elegido).

```
./target/release/polytabletop host chess               # un jugador aloja la partida
./target/release/polytabletop join 192.168.1.20:7878   # el otro se une con la dirección y el puerto del anfitrión
```

## Idiomas (`--lang`)

La interfaz habla 8 idiomas: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (se elige con `--lang`, luego `POLYTABLETOP_LANG`, luego el `LANG` del sistema; si no, `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

Más información: [README](../README.es.md).
