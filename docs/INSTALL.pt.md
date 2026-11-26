# Instalação e arranque rápido

[English](INSTALL.en.md) · [Română](INSTALL.ro.md) · [Français](INSTALL.fr.md) · [Italiano](INSTALL.it.md) · [Español](INSTALL.es.md) · [Português](INSTALL.pt.md) · [Deutsch](INSTALL.de.md) · [Magyar](INSTALL.hu.md)

## O que é

**PolyTabletop** é uma coleção de jogos de tabuleiro (xadrez, go, damas, jogos de cartas, de dados e muitos outros) escritos de raiz em Rust, que se jogam no terminal, numa interface gráfica e em rede local. O texto em romeno é o de referência; esta página é um guia breve.

## Instalação

- **A partir de um arquivo de lançamento.** Descarrega o arquivo para a tua plataforma na página Releases do projeto, descompacta-o onde quiseres (não precisa de instalador nem de bibliotecas externas) e executa o ficheiro `polytabletop` a partir dessa pasta.
- **Com o script de instalação (Linux e macOS).** `bash install.sh [etiqueta-da-versão]` (script do repositório `chiuta/PolyTabletop`) descarrega o último arquivo de lançamento para a tua plataforma, verifica a soma SHA-256 e instala `polytabletop`, `polytabletop-gui` e `polytabletop-rpg` em `~/.local/bin` (altera-se com `PREFIX=...`), além da página de manual e do alias curto `ptt`. No Windows, descarrega o arquivo `.zip` em Releases.
- **A partir do código-fonte com cargo** (Rust, cadeia de ferramentas estável recente):

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess
```

Interface gráfica (opcional): `cargo run --release -p polytabletop-gui`.

## Primeiro jogo

`polytabletop list` mostra os jogos; `polytabletop play <jogo>` inicia um. Escreve a jogada (ex.: `e2e4`), `?` = jogadas legais, `hint` = sugestão da IA, `q` = sair.

## No mesmo computador ou contra a IA

```
./target/release/polytabletop play chess --you one     # jogas como Jogador 1 (predefinição)
./target/release/polytabletop play chess --you two     # jogas como Jogador 2 (o computador começa)
./target/release/polytabletop play chess --you both    # à vez no mesmo computador: os dois jogadores no mesmo teclado
./target/release/polytabletop play go --you two
```

## Partidas reproduzíveis (`--seed`)

`--seed N` torna reproduzíveis a distribuição das cartas e os dados: o mesmo número dá a mesma partida.

```
./target/release/polytabletop play barbut --seed 42
```

## Rede local: alojar e juntar-se

Em rede local, um computador aloja a partida e o outro liga-se a ele. Os dois computadores têm de se alcançar mutuamente (firewall aberta na porta escolhida).

```
./target/release/polytabletop host chess               # um jogador aloja a partida
./target/release/polytabletop join 192.168.1.20:7878   # o outro junta-se com o endereço e a porta do anfitrião
```

## Idiomas (`--lang`)

A interface fala 8 idiomas: `en`, `ro`, `fr`, `it`, `es`, `pt`, `de`, `hu` (escolhida por `--lang`, depois `POLYTABLETOP_LANG`, depois o `LANG` do sistema; caso contrário, `ro`).

```
./target/release/polytabletop --lang fr play chess
./target/release/polytabletop --lang hu list
```

Mais informação: [README](../README.pt.md).
