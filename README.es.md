[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 0.9.1 · Last updated: 2026-11-26

## Qué es

PolyTabletop es una colección clean-room de juegos de mesa escrita en Rust: 171 juegos que se pueden jugar contra el ordenador, en la terminal o en una ventana gráfica, además del modo hot-seat en un mismo equipo y el juego en red para dos personas. Hay clásicos abstractos, juegos de cartas y de dados, variantes originales de mecánicas modernas con nombres propios, juegos cooperativos con un compañero de IA, juegos de roles ocultos en los que los robots completan la mesa, juegos de fiesta y de palabras, y motores de miniaturas y de cartas coleccionables con cartas originales. El motor y los juegos no tienen dependencias externas; solo la interfaz gráfica (`polytabletop-gui`) usa `eframe`/egui. El catálogo recoge 251 juegos objetivo.

Página de presentación: <https://polytabletop.chiuta.ro>. Guía breve de instalación: [docs/INSTALL.es.md](docs/INSTALL.es.md) ([English](docs/INSTALL.en.md) · [Română](docs/INSTALL.ro.md) · [Français](docs/INSTALL.fr.md) · [Italiano](docs/INSTALL.it.md) · [Português](docs/INSTALL.pt.md) · [Deutsch](docs/INSTALL.de.md) · [Magyar](docs/INSTALL.hu.md)).

## Uso

- Lista y catálogo: `polytabletop list` muestra los 171 juegos jugables con sus reglas resumidas; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` muestra el estado de las 251 entradas del catálogo.
- Jugar: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (predeterminado) te hace Jugador 1, `two` Jugador 2, `both` es el modo hot-seat con un solo teclado, y `none` deja que el ordenador juegue los dos bandos. `--seed N` hace reproducibles el reparto de cartas y los dados; `--record ARCHIVO` guarda una transcripción.
- Durante la partida: escribe la jugada exactamente como la lista `?` (por ejemplo `e2e4`, `c3xe5`, `d6`), `?` = jugadas legales, `hint` = sugerencia de la IA, `q` = salir.
- IA contra IA: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (véase [docs/ARENA.md](docs/ARENA.md)).
- Dos personas en red: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` y `polytabletop join HOST:PUERTO`.
- Transcripciones: `polytabletop replay ARCHIVO [--step]`, `polytabletop export ARCHIVO --format pgn|text|json` (PGN solo para ajedrez), `polytabletop import-pgn ARCHIVO [--out TRANSCRIPCION]`.
- Idiomas: `polytabletop langs` lista los 8 idiomas de la interfaz; `--lang xx` puede ir en cualquier lugar de la línea de comandos.
- Interfaz gráfica: `cargo run --release -p polytabletop-gui` (`-- --selftest` ejecuta las comprobaciones sin pantalla). Kit para juegos de rol: `cargo run --release -p polytabletop-rpg`.
- Instalados por `install.sh`: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, la página de manual y el alias corto `ptt` (`ptt list` = `polytabletop list`).
- Variables de entorno: `POLYTABLETOP_LANG` (idioma), `POLYTABLETOP_LEVEL` (easy, normal, hard o expert), `POLYTABLETOP_AI_MS` (presupuesto de tiempo de la IA, en milisegundos).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # eres el Jugador 1 (las blancas)
./target/release/polytabletop play go --you two      # empieza el ordenador
./target/release/polytabletop --lang de play barbut --seed 42
```

## Juegos

Ya se pueden jugar 171 juegos, en la terminal y en la interfaz gráfica; el catálogo tiene 251 entradas (`polytabletop catalog` termina con: implementadas 182, juegos tradicionales previstos 50, variantes propias previstas 3, kit genérico 16). Las 16 entradas de kit son sistemas de juego de rol, atendidas por el crate genérico `polytabletop-rpg` (dados, fichas, pruebas de habilidad, generador de mazmorras, simulador de batallas, diario), sin las reglas de ninguna editorial. Categorías y número de juegos, según `polytabletop-games/src/lib.rs`:

| Categoría | Juegos | Ejemplos (id) |
|---|---|---|
| Cartas | 45 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda` |
| Abstractos | 40 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane` |
| Estrategia | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, y variantes originales de juegos modernos |
| Fiesta (party) | 16 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Dados | 11 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut` |
| Clásicos | 9 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo` |
| Familiares | 7 | `orasel`, `roata`, `turnul`, `zidurile` |
| Miniaturas | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Cooperativos | 6 | `artificii`, `focare`, `strajile`, `cripta` |
| Cartas coleccionables (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Deducción | 1 | `conac` |
| Palabras | 1 | `litere` |
| **Total** | **171** | |

Tres entradas del catálogo siguen previstas como variantes originales (Twister, Campionii Distracției, Turistico). Las reglas resumidas de cada juego están en [docs/JOCURI.md](docs/JOCURI.md) y el catálogo generado en [docs/CATALOG.md](docs/CATALOG.md). En la interfaz gráfica, 36 juegos tienen un tablero dibujado con jugadas por clic (ajedrez, damas, reversi, cuatro en línea, gomoku, go, el molino, xiangqi, shogi, backgammon, ludo y pachisi, hex, la familia mancala y más); los demás juegos aparecen como texto, con las jugadas legales como botones.

## Idiomas

PolyTabletop habla 8 idiomas: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. Las interfaces de línea de comandos y gráfica están traducidas a los 8. El idioma se elige con `--lang xx`, luego `POLYTABLETOP_LANG`, luego el `LANG` del sistema; si no, rumano. Los títulos y las reglas de los 171 juegos están traducidos a los 8 idiomas (`polytabletop-games/data/i18n/`, con recurso al texto rumano si falta una entrada); el texto dentro de los juegos (cartas, bancos de contenido, avisos) sigue en rumano. La notación de las jugadas (`e2e4`, `c3-d4`) nunca cambia. Detalles: [docs/I18N.md](docs/I18N.md).

## IA

- Los juegos abstractos usan búsqueda alfa-beta con profundización iterativa, presupuesto de tiempo, tabla de transposición y quiescencia; los juegos de dados, de cartas y con gran factor de ramificación usan MCTS (UCT). Detalles en [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Los niveles `easy`, `normal`, `hard`, `expert` escalan el presupuesto de tiempo y el número de simulaciones (aproximadamente 0,1x, 1x, 2,5x y 6x); en `easy` la IA a veces juega a propósito una jugada más débil.
- La IA no hace trampa: su evaluación y sus simulaciones solo leen la información visible para el jugador por el que juega; en los juegos con manos ocultas sortea al azar la parte desconocida en lugar de mirar la mano del adversario.
- Información oculta y hot-seat: en el modo `--you both`, los juegos de `HOTSEAT_SAFE` muestran solo la información privada de quien tiene el turno; para los demás juegos con estado oculto aleatorio aparece un aviso y `hint` queda desactivado. Cada juego pertenece a una de tres clases (seguro, un humano contra robots con vista total para un segundo humano, o sin información privada), comprobada por una prueba. Véase [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Juego en red

`polytabletop host ID` espera a un cliente en el puerto 7878 (o `--port P`) y juega como Jugador 1 (o 2, o al azar con `--side`); `polytabletop join HOST:PUERTO` se conecta. El protocolo de texto `polytabletop/1` es autoritativo en el servidor: allí vive la única copia de la partida, y cada lado ve solo su información privada. Quien no mueve en `--timeout` segundos (600 por defecto) pierde. En la terminal se escribe una jugada, `?`, `state`, `chat TEXTO` o `q`. Protocolo y detalles: [docs/RETEA.md](docs/RETEA.md). El juego en red está probado en bucle local (`polytabletop/tests/net_loopback.rs`), no en redes reales.

## Páginas de reglas

Cada juego jugable tiene una página con el objetivo, la preparación, la estructura del turno, la sintaxis de las jugadas, las reglas especiales, la puntuación, el final de la partida, notas sobre la IA y diferencias con las reglas tradicionales: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (juegos 1 a 86) y [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (juegos 87 a 171). Las páginas están en rumano.

## Arquitectura

| Crate | Función |
|---|---|
| `polytabletop-core` | trait tipado `Game`, trait `GameObj` apto para objetos (jugadas como texto, para cualquier interfaz), IA (alfa-beta, MCTS), generador aleatorio propio, i18n; sin dependencias externas |
| `polytabletop-games` | todos los juegos (un módulo cada uno), `playable()`, `HOTSEAT_SAFE`, catálogo generado; sin dependencias externas |
| `polytabletop-rpg` | kit genérico para juegos de rol; sin dependencias externas |
| `polytabletop-gui` | interfaz gráfica egui/eframe; el único crate con dependencias externas, fuera de la compilación predeterminada |
| `polytabletop` | interfaz de línea de comandos, arena, red, transcripciones |

Un juego nuevo es una estructura que implementa `Game` (jugadas legales, aplicación, resultado, evaluación, visualización) más una entrada en `playable()`; cualquier interfaz lo recibe gratis. Véanse [docs/ARHITECTURA.md](docs/ARHITECTURA.md) y [docs/GUI.md](docs/GUI.md) (documentos en rumano).

## Compilación, pruebas y CI

- Compilación: `cargo build --release` (Rust, cadena de herramientas estable reciente). La interfaz gráfica no forma parte de la compilación predeterminada: `cargo build --release -p polytabletop-gui`; en Linux necesita las bibliotecas de desarrollo de X11 o Wayland.
- Pruebas: `cargo test --release` (perft en ajedrez 20 / 400 / 8.902 / 197.281, reglas especiales, partidas aleatorias completas de cada juego, IA contra jugadas aleatorias, auditoría del punto de vista, coherencia del catálogo, red en bucle local).
- Antes de una pull request: `cargo fmt --all` y `cargo clippy --release --all-targets -- -D warnings`.
- La CI (`.github/workflows/ci.yml`) ejecuta fmt, clippy y las pruebas en Ubuntu 26.04, y luego compila la interfaz gráfica y ejecuta `polytabletop-gui --selftest`. Una etiqueta `v*` lanza `release.yml`, que construye archivos para Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) con sumas SHA-256; `install.sh` los instala desde `chiuta/PolyTabletop`. Se incluye también un `Dockerfile` (solo CLI y RPG).

## Clean-room y marcas

- Las reglas proceden solo de descripciones públicas de los juegos; no se usan código, textos, gráficos ni datos de productos comerciales, y las fuentes públicas se anotan en la cabecera de cada archivo de juego.
- La mecánica de un juego es libre, su nombre, su arte y sus textos no: los juegos llevan nombres genéricos («Reversi», «Cuatro en línea») o, en los juegos modernos con marca, variantes originales con títulos, temas y contenido propios.
- Sin marcas en los id, títulos, nombres de cartas ni bancos de contenido. Los nombres de marcas aparecen solo en el catálogo, como objetivos de planificación.
- Los juegos de rol, de miniaturas y de cartas coleccionables son kits genéricos (dados, fichas, mazos originales), nunca una copia de las reglas o del contenido de una editorial.

## Límites

- La interfaz gráfica solo se ha probado sin pantalla (`polytabletop-gui --selftest`); no se ha comprobado visualmente en una pantalla real.
- La fuerza de la IA se ha medido contra jugadas aleatorias y entre presupuestos de tiempo, no contra jugadores humanos; algunos juegos tienen límites conocidos (por ejemplo, la tasa de victorias de `macao` y `hruba` figura en el changelog).
- Las reglas de los juegos escritas de memoria se compararon con fuentes públicas, no con cada reglamento oficial, y los juegos no los han probado jugadores externos.
- El juego en red solo está probado en bucle local.
- Los títulos y las reglas de los juegos están traducidos, pero el texto dentro de los juegos (cartas, bancos de contenido, avisos) sigue en rumano; las traducciones no han sido revisadas por hablantes nativos.
- Las 16 entradas de juegos de rol las atiende un kit genérico, no implementaciones de esos sistemas.

## Contribuir e historial

Se agradecen las contribuciones dentro de las reglas clean-room: véase [CONTRIBUTING.md](CONTRIBUTING.md) (cómo añadir un juego, pruebas, lista de comprobación para pull requests). Historial de versiones: [CHANGELOG.md](CHANGELOG.md).

## Licencia

Declarada en `Cargo.toml` como `MIT OR Apache-2.0`; textos completos en [LICENSE-MIT](LICENSE-MIT) y [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Créditos

Autor: Alexio (Alexandru-Ionuț Chiuță), de la serie «Fără Frontiere» (Sin Fronteras) de aplicaciones trade-free. Contacto: alexio@trom.tf. Página de presentación: <https://polytabletop.chiuta.ro>.
