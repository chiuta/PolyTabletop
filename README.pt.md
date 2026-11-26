[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 0.9.1 · Last updated: 2026-11-26

## O que é

O PolyTabletop é uma coleção clean-room de jogos de tabuleiro e de mesa escrita em Rust: 171 jogos que se podem jogar contra o computador, no terminal ou numa janela gráfica, além do modo hot-seat no mesmo computador e do jogo em rede para duas pessoas. Há clássicos abstratos, jogos de cartas e de dados, variantes originais de mecânicas modernas com nomes próprios, jogos cooperativos com um parceiro de IA, jogos de papéis ocultos em que os robôs completam a mesa, jogos de festa e de palavras, e motores de miniaturas e de cartas colecionáveis com cartas originais. O motor e os jogos não têm dependências externas; só a interface gráfica (`polytabletop-gui`) usa `eframe`/egui. O catálogo acompanha 251 jogos-alvo.

Página de apresentação: <https://polytabletop.chiuta.ro>. Guia rápido de instalação: [docs/INSTALL.pt.md](docs/INSTALL.pt.md) ([English](docs/INSTALL.en.md) · [Română](docs/INSTALL.ro.md) · [Français](docs/INSTALL.fr.md) · [Italiano](docs/INSTALL.it.md) · [Español](docs/INSTALL.es.md) · [Deutsch](docs/INSTALL.de.md) · [Magyar](docs/INSTALL.hu.md)).

## Utilização

- Lista e catálogo: `polytabletop list` mostra os 171 jogos jogáveis com as regras resumidas; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` mostra o estado das 251 entradas do catálogo.
- Jogar: `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (predefinição) faz de ti o Jogador 1, `two` o Jogador 2, `both` é o modo hot-seat com um só teclado, e `none` deixa o computador jogar os dois lados. `--seed N` torna reproduzíveis a distribuição das cartas e os dados; `--record FICHEIRO` guarda uma transcrição.
- Durante o jogo: escreve a jogada exatamente como o `?` a lista (por exemplo `e2e4`, `c3xe5`, `d6`), `?` = jogadas legais, `hint` = sugestão da IA, `q` = sair.
- IA contra IA: `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (ver [docs/ARENA.md](docs/ARENA.md)).
- Duas pessoas em rede: `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` e `polytabletop join HOST:PORTA`.
- Transcrições: `polytabletop replay FICHEIRO [--step]`, `polytabletop export FICHEIRO --format pgn|text|json` (PGN só para xadrez), `polytabletop import-pgn FICHEIRO [--out TRANSCRIÇÃO]`.
- Idiomas: `polytabletop langs` lista os 8 idiomas da interface; `--lang xx` pode estar em qualquer ponto da linha de comandos.
- Interface gráfica: `cargo run --release -p polytabletop-gui` (`-- --selftest` executa as verificações sem ecrã). Kit para jogos de interpretação de papéis: `cargo run --release -p polytabletop-rpg`.
- Instalados pelo `install.sh`: `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, a página de manual e o alias curto `ptt` (`ptt list` = `polytabletop list`).
- Variáveis de ambiente: `POLYTABLETOP_LANG` (idioma), `POLYTABLETOP_LEVEL` (easy, normal, hard ou expert), `POLYTABLETOP_AI_MS` (orçamento de tempo da IA, em milissegundos).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # és o Jogador 1 (as brancas)
./target/release/polytabletop play go --you two      # o computador começa
./target/release/polytabletop --lang de play barbut --seed 42
```

## Jogos

Já se podem jogar 171 jogos, no terminal e na interface gráfica; o catálogo tem 251 entradas (`polytabletop catalog` termina com: implementadas 182, jogos tradicionais previstos 50, variantes próprias previstas 3, kit genérico 16). As 16 entradas de kit são sistemas de jogos de papéis, servidas pelo crate genérico `polytabletop-rpg` (dados, fichas, testes de perícia, gerador de masmorras, simulador de batalhas, diário), sem as regras de qualquer editora. Categorias e número de jogos, a partir de `polytabletop-games/src/lib.rs`:

| Categoria | Jogos | Exemplos (id) |
|---|---|---|
| Cartas | 45 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda` |
| Abstratos | 40 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane` |
| Estratégia | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, e variantes originais de jogos modernos |
| Festa (party) | 16 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Dados | 11 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut` |
| Clássicos | 9 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo` |
| Família | 7 | `orasel`, `roata`, `turnul`, `zidurile` |
| Miniaturas | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Cooperativos | 6 | `artificii`, `focare`, `strajile`, `cripta` |
| Cartas colecionáveis (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Dedução | 1 | `conac` |
| Palavras | 1 | `litere` |
| **Total** | **171** | |

Três entradas do catálogo continuam previstas como variantes originais (Twister, Campionii Distracției, Turistico). As regras resumidas de cada jogo estão em [docs/JOCURI.md](docs/JOCURI.md) e o catálogo gerado em [docs/CATALOG.md](docs/CATALOG.md). Na interface gráfica, 36 jogos têm tabuleiro desenhado com jogadas por clique (xadrez, damas, reversi, quatro em linha, gomoku, go, moinho, xiangqi, shogi, gamão, ludo e pachisi, hex, a família mancala e outros); os restantes aparecem em texto, com as jogadas legais como botões.

## Idiomas

O PolyTabletop fala 8 idiomas: English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. As interfaces de linha de comandos e gráfica estão traduzidas nos 8. O idioma escolhe-se por `--lang xx`, depois `POLYTABLETOP_LANG`, depois o `LANG` do sistema; caso contrário, romeno. Os títulos e as regras dos 171 jogos estão traduzidos nos 8 idiomas (`polytabletop-games/data/i18n/`, com recurso ao texto romeno se faltar uma entrada); o texto dentro dos jogos (cartas, bancos de conteúdo, mensagens) continua em romeno. A notação das jogadas (`e2e4`, `c3-d4`) nunca muda. Detalhes: [docs/I18N.md](docs/I18N.md).

## IA

- Os jogos abstratos usam pesquisa alfa-beta com aprofundamento iterativo, orçamento de tempo, tabela de transposição e quiescência; os jogos de dados, de cartas e com grande fator de ramificação usam MCTS (UCT). Detalhes em [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Os níveis `easy`, `normal`, `hard`, `expert` ajustam o orçamento de tempo e o número de simulações (cerca de 0,1x, 1x, 2,5x e 6x); em `easy` a IA joga por vezes, de propósito, uma jogada mais fraca.
- A IA não faz batota: a avaliação e as simulações só leem a informação visível para o jogador por quem joga; nos jogos com mãos ocultas sorteia ao acaso a parte desconhecida, em vez de olhar para a mão do adversário.
- Informação oculta e hot-seat: no modo `--you both`, os jogos listados em `HOTSEAT_SAFE` mostram apenas a informação privada de quem tem a vez; para os outros jogos com estado oculto aleatório aparece um aviso e o `hint` fica desativado. Cada jogo pertence a uma de três classes (seguro, um humano contra robôs com vista total para um segundo humano, ou sem informação privada), verificada por um teste. Ver [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Jogo em rede

`polytabletop host ID` espera um cliente na porta 7878 (ou `--port P`) e joga como Jogador 1 (ou 2, ou ao acaso com `--side`); `polytabletop join HOST:PORTA` liga-se. O protocolo de texto `polytabletop/1` é autoritário no servidor: lá está a única cópia do jogo, e cada lado vê apenas a sua informação privada. Quem não jogar em `--timeout` segundos (600 por predefinição) perde. No terminal escreve-se uma jogada, `?`, `state`, `chat TEXTO` ou `q`. Protocolo e detalhes: [docs/RETEA.md](docs/RETEA.md). O jogo em rede está testado em loopback (`polytabletop/tests/net_loopback.rs`), não em redes reais.

## Páginas de regras

Cada jogo jogável tem uma página com o objetivo, a preparação, a estrutura do turno, a sintaxe das jogadas, as regras especiais, a pontuação, o fim do jogo, notas sobre a IA e diferenças em relação às regras tradicionais: [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (jogos 1 a 86) e [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (jogos 87 a 171). As páginas estão em romeno.

## Arquitetura

| Crate | Função |
|---|---|
| `polytabletop-core` | trait tipado `Game`, trait `GameObj` utilizável como objeto (jogadas como texto, para qualquer interface), IA (alfa-beta, MCTS), gerador aleatório próprio, i18n; sem dependências externas |
| `polytabletop-games` | todos os jogos (um módulo cada), `playable()`, `HOTSEAT_SAFE`, catálogo gerado; sem dependências externas |
| `polytabletop-rpg` | kit genérico para jogos de papéis; sem dependências externas |
| `polytabletop-gui` | interface gráfica egui/eframe; o único crate com dependências externas, fora da compilação predefinida |
| `polytabletop` | interface de linha de comandos, arena, rede, transcrições |

Um jogo novo é uma estrutura que implementa `Game` (jogadas legais, aplicação, resultado, avaliação, apresentação) mais uma entrada em `playable()`; qualquer interface o recebe de graça. Ver [docs/ARHITECTURA.md](docs/ARHITECTURA.md) e [docs/GUI.md](docs/GUI.md) (documentos em romeno).

## Compilação, testes e CI

- Compilação: `cargo build --release` (Rust, cadeia de ferramentas estável recente). A interface gráfica não faz parte da compilação predefinida: `cargo build --release -p polytabletop-gui`; no Linux precisa das bibliotecas de desenvolvimento X11 ou Wayland.
- Testes: `cargo test --release` (perft no xadrez 20 / 400 / 8 902 / 197 281, regras especiais, partidas aleatórias completas de cada jogo, IA contra jogadas aleatórias, auditoria do ponto de vista, coerência do catálogo, rede em loopback).
- Antes de um pull request: `cargo fmt --all` e `cargo clippy --release --all-targets -- -D warnings`.
- A CI (`.github/workflows/ci.yml`) executa fmt, clippy e os testes no Ubuntu 26.04, depois compila a interface gráfica e executa `polytabletop-gui --selftest`. Uma etiqueta `v*` aciona o `release.yml`, que constrói arquivos para Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) com somas SHA-256; o `install.sh` instala-os a partir de `chiuta/PolyTabletop`. Inclui-se também um `Dockerfile` (apenas CLI e RPG).

## Clean-room e marcas

- As regras vêm apenas de descrições públicas dos jogos; não se usam código, textos, gráficos nem dados de produtos comerciais, e as fontes públicas ficam anotadas no cabeçalho de cada ficheiro de jogo.
- A mecânica de um jogo é livre, o nome, a arte e os textos não: os jogos têm nomes genéricos («Reversi», «Quatro em linha») ou, nos jogos modernos com marca, variantes originais com títulos, temas e conteúdo próprios.
- Sem marcas nos id, títulos, nomes de cartas ou bancos de conteúdo. Os nomes de marcas só aparecem no catálogo, como alvos de planeamento.
- Os jogos de papéis, de miniaturas e de cartas colecionáveis são kits genéricos (dados, fichas, baralhos originais), nunca uma cópia das regras ou do conteúdo de uma editora.

## Limites

- A interface gráfica só foi testada sem ecrã (`polytabletop-gui --selftest`); não foi verificada visualmente num ecrã real.
- A força da IA foi medida contra jogadas aleatórias e entre orçamentos de tempo, não contra jogadores humanos; alguns jogos têm limites conhecidos (por exemplo, a taxa de vitórias de `macao` e `hruba` consta do changelog).
- As regras dos jogos escritas de memória foram comparadas com fontes públicas, não com cada regulamento oficial, e os jogos não foram testados por jogadores externos.
- O jogo em rede só está testado em loopback.
- Os títulos e as regras dos jogos estão traduzidos, mas o texto dentro dos jogos (cartas, bancos de conteúdo, mensagens) continua em romeno; as traduções não foram revistas por falantes nativos.
- As 16 entradas de jogos de papéis são servidas por um kit genérico, não por implementações desses sistemas.

## Contribuir e histórico

As contribuições são bem-vindas dentro das regras clean-room: ver [CONTRIBUTING.md](CONTRIBUTING.md) (como adicionar um jogo, testes, lista de verificação para pull requests). Histórico de versões: [CHANGELOG.md](CHANGELOG.md).

## Licença

Declarada em `Cargo.toml` como `MIT OR Apache-2.0`; textos completos em [LICENSE-MIT](LICENSE-MIT) e [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Créditos

Autor: Alexio (Alexandru-Ionuț Chiuță), da série «Fără Frontiere» (Sem Fronteiras) de aplicações trade-free. Contacto: alexio@trom.tf. Página de apresentação: <https://polytabletop.chiuta.ro>.
