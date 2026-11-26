[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

# **PolyTabletop**

Version: 0.9.1 · Last updated: 2026-11-26

## De quoi s'agit-il

PolyTabletop est une collection clean-room de jeux de société écrite en Rust : 171 jeux jouables contre l'ordinateur, dans le terminal ou dans une fenêtre graphique, avec en plus le mode « à deux sur le même poste » (hot-seat) et le jeu en réseau à deux. On y trouve des classiques abstraits, des jeux de cartes et de dés, des variantes originales de mécaniques modernes sous des noms propres, des jeux coopératifs avec un partenaire IA, des jeux à rôles cachés où des robots complètent la table, des jeux d'ambiance et de mots, ainsi que des moteurs de figurines et de cartes à collectionner avec des cartes originales. Le moteur et les jeux n'ont aucune dépendance externe ; seule l'interface graphique (`polytabletop-gui`) utilise `eframe`/egui. Le catalogue suit 1200 jeux cibles.

Page de présentation : <https://polytabletop.chiuta.ro>. Guide d'installation court : [docs/INSTALL.fr.md](docs/INSTALL.fr.md) ([English](docs/INSTALL.en.md) · [Română](docs/INSTALL.ro.md) · [Italiano](docs/INSTALL.it.md) · [Español](docs/INSTALL.es.md) · [Português](docs/INSTALL.pt.md) · [Deutsch](docs/INSTALL.de.md) · [Magyar](docs/INSTALL.hu.md)).

## Utilisation

- Liste et catalogue : `polytabletop list` affiche les 171 jeux jouables avec leurs règles en bref ; `polytabletop catalog [--kind implemented|traditional|branded|toolkit]` donne l'état des 1200 entrées du catalogue.
- Jouer : `polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert] [--lang xx]`. `--you one` (par défaut) vous fait Joueur 1, `two` Joueur 2, `both` est le mode hot-seat sur un seul clavier, `none` laisse l'ordinateur jouer les deux camps. `--seed N` rend la distribution des cartes et les dés reproductibles ; `--record FICHIER` enregistre une transcription.
- En partie : saisissez le coup exactement comme `?` le liste (par exemple `e2e4`, `c3xe5`, `d6`), `?` = coups légaux, `hint` = suggestion de l'IA, `q` = quitter.
- IA contre IA : `polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]`, `polytabletop bench`, `polytabletop ladder ID` (voir [docs/ARENA.md](docs/ARENA.md)).
- Deux personnes en réseau : `polytabletop host ID [--port P] [--seed N] [--side 1|2|random]` et `polytabletop join HOTE:PORT`.
- Transcriptions : `polytabletop replay FICHIER [--step]`, `polytabletop export FICHIER --format pgn|text|json` (PGN seulement pour les échecs), `polytabletop import-pgn FICHIER [--out TRANSCRIPTION]`.
- Langues : `polytabletop langs` liste les 8 langues de l'interface ; `--lang xx` peut figurer n'importe où dans la ligne de commande.
- Interface graphique : `cargo run --release -p polytabletop-gui` (`-- --selftest` lance les vérifications sans écran). Boîte à outils pour jeux de rôle : `cargo run --release -p polytabletop-rpg`.
- Installés par `install.sh` : `polytabletop`, `polytabletop-gui`, `polytabletop-rpg`, la page de manuel et l'alias court `ptt` (`ptt list` = `polytabletop list`).
- Variables d'environnement : `POLYTABLETOP_LANG` (langue), `POLYTABLETOP_LEVEL` (easy, normal, hard ou expert), `POLYTABLETOP_AI_MS` (budget de temps de l'IA, en millisecondes).

```
cargo build --release
./target/release/polytabletop list
./target/release/polytabletop play chess             # vous êtes le Joueur 1 (les blancs)
./target/release/polytabletop play go --you two      # l'ordinateur commence
./target/release/polytabletop --lang de play barbut --seed 42
```

## Jeux

171 jeux sont jouables dès maintenant, dans le terminal et dans l'interface graphique ; le catalogue compte 1200 entrées (`polytabletop catalog` se termine par : implémentées 172, jeux traditionnels prévus 263, variantes propres prévues 477, boîte à outils générique 288). Les 288 entrées « boîte à outils » sont les 100 systèmes de jeu de rôle, servis par le crate générique `polytabletop-rpg` (dés, fiches, tests de compétence, générateur de donjon, simulateur de batailles, journal), plus les 93 titres de figurines et 95 de cartes à collectionner pas encore dans le programme, tous sans les règles d'aucun éditeur. Chacune des douze catégories du catalogue compte exactement 100 entrées distinctes. Catégories et nombre de jeux, d'après `polytabletop-games/src/lib.rs` :

| Catégorie | Jeux | Exemples (identifiants) |
|---|---|---|
| Cartes | 45 | `septica`, `macao`, `belote`, `bridge`, `poker`, `rummy`, `tarot`, `skat`, `briscola`, `cribbage`, `hanafuda` |
| Abstrait | 40 | `chess`, `draughts`, `reversi`, `go`, `xiangqi`, `shogi`, `hex`, `gomoku`, `mancala`, `konane` |
| Stratégie | 24 | `insula`, `mozaic`, `arhipelag`, `apus`, `gospodaria`, `furnale`, ainsi que des variantes originales de jeux modernes |
| Ambiance (party) | 16 | `lupii`, `consiliul`, `spionii`, `povestitor`, `impostor` |
| Dés | 11 | `backgammon`, `ludo`, `snakes`, `barbut`, `zarpoker`, `yut` |
| Classique | 9 | `domino`, `mahjong`, `battleship`, `mastermind`, `bingo` |
| Famille | 7 | `orasel`, `roata`, `turnul`, `zidurile` |
| Figurines | 6 | `miniaturi`, `nave`, `patrula`, `gridiron` |
| Coopératif | 6 | `artificii`, `focare`, `strajile`, `cripta` |
| Cartes à collectionner (TCG) | 5 | `tcg`, `tcg_evo`, `tcg_zone`, `tcg_lant`, `tcg_cerneala` |
| Déduction | 1 | `conac` |
| Mots | 1 | `litere` |
| **Total** | **171** | |

477 entrées du catalogue sont prévues comme variantes originales sous leur propre titre (par exemple Twister, Campionii Distracției, Turistico) et 263 comme jeux traditionnels aux règles publiques. Les règles abrégées de chaque jeu figurent dans [docs/JOCURI.md](docs/JOCURI.md), le catalogue généré dans [docs/CATALOG.md](docs/CATALOG.md). Dans l'interface graphique, 36 jeux ont un plateau dessiné avec des coups au clic (échecs, dames, reversi, puissance quatre, gomoku, go, moulin, xiangqi, shogi, backgammon, ludo et pachisi, hex, la famille mancala et d'autres) ; les autres jeux s'affichent en texte, leurs coups légaux étant proposés comme boutons.

## Langues

PolyTabletop parle 8 langues : English, Română, Français, Italiano, Español, Português, Deutsch, Magyar. Les interfaces en ligne de commande et graphique sont traduites dans les 8. La langue est choisie par `--lang xx`, puis `POLYTABLETOP_LANG`, puis le `LANG` du système, sinon le roumain. Les titres et les règles des 171 jeux sont traduits dans les 8 langues (`polytabletop-games/data/i18n/`, avec repli sur le texte roumain si une entrée manque) ; le texte à l'intérieur des jeux (cartes, banques de contenu, invites) est encore en roumain. La notation des coups (`e2e4`, `c3-d4`) ne change jamais. Détails : [docs/I18N.md](docs/I18N.md).

## IA

- Les jeux abstraits utilisent une recherche alpha-bêta avec approfondissement itératif, budget de temps, table de transposition et quiescence ; les jeux de dés, de cartes et à fort facteur de branchement utilisent MCTS (UCT). Détails dans [docs/ARHITECTURA.md](docs/ARHITECTURA.md).
- Les niveaux `easy`, `normal`, `hard`, `expert` modulent le budget de temps et le nombre de simulations (environ 0,1x, 1x, 2,5x et 6x) ; à `easy`, l'IA joue parfois volontairement un coup plus faible.
- L'IA ne triche pas : son évaluation et ses simulations ne lisent que l'information visible du joueur pour lequel elle joue ; dans les jeux à mains cachées, elle tire au hasard la partie inconnue au lieu de regarder la main de l'adversaire.
- Information cachée et hot-seat : en mode `--you both`, les jeux listés dans `HOTSEAT_SAFE` n'affichent que l'information privée du joueur dont c'est le tour ; pour les autres jeux à état caché aléatoire, un avertissement s'affiche et `hint` est désactivé. Chaque jeu relève d'une de trois catégories (sûr, un humain contre des robots avec vue totale pour un second humain, ou sans information privée), vérifiée par un test. Voir [docs/HOTSEAT.md](docs/HOTSEAT.md).

## Jeu en réseau

`polytabletop host ID` attend un client sur le port 7878 (ou `--port P`) et joue lui-même le Joueur 1 (ou 2, ou au hasard avec `--side`) ; `polytabletop join HOTE:PORT` s'y connecte. Le protocole texte `polytabletop/1` fait autorité côté serveur : l'unique copie de la partie y réside, et chaque côté ne voit que son information privée. Celui qui ne joue pas dans les `--timeout` secondes (600 par défaut) perd. Dans le terminal, on saisit un coup, `?`, `state`, `chat TEXTE` ou `q`. Protocole et détails : [docs/RETEA.md](docs/RETEA.md). Le jeu en réseau est testé en boucle locale (`polytabletop/tests/net_loopback.rs`), pas sur de vrais réseaux.

## Pages de règles

Chaque jeu jouable a une page avec le but, la mise en place, le déroulement d'un tour, la syntaxe des coups, les règles spéciales, le décompte des points, la fin de partie, des notes sur l'IA et les différences avec les règles traditionnelles : [docs/reguli/INDEX.md](docs/reguli/INDEX.md) (jeux 1 à 86) et [docs/reguli/INDEX2.md](docs/reguli/INDEX2.md) (jeux 87 à 171). Les pages sont en roumain.

## Architecture

| Crate | Rôle |
|---|---|
| `polytabletop-core` | trait typé `Game`, trait `GameObj` compatible objet (coups en texte, pour toute interface), IA (alpha-bêta, MCTS), générateur aléatoire propre, i18n ; sans dépendance externe |
| `polytabletop-games` | tous les jeux (un module chacun), `playable()`, `HOTSEAT_SAFE`, catalogue généré ; sans dépendance externe |
| `polytabletop-rpg` | boîte à outils générique pour jeux de rôle ; sans dépendance externe |
| `polytabletop-gui` | interface graphique egui/eframe ; seul crate avec des dépendances externes, hors de la compilation par défaut |
| `polytabletop` | interface en ligne de commande, arène, réseau, transcriptions |

Un nouveau jeu est une structure qui implémente `Game` (coups légaux, application, résultat, évaluation, affichage) plus une entrée dans `playable()` ; toute interface le reçoit gratuitement. Voir [docs/ARHITECTURA.md](docs/ARHITECTURA.md) et [docs/GUI.md](docs/GUI.md) (documents en roumain).

## Compilation, tests et CI

- Compilation : `cargo build --release` (Rust, chaîne d'outils stable récente). L'interface graphique ne fait pas partie de la compilation par défaut : `cargo build --release -p polytabletop-gui` ; sous Linux, elle exige les bibliothèques de développement X11 ou Wayland.
- Tests : `cargo test --release` (perft aux échecs 20 / 400 / 8 902 / 197 281, règles spéciales, parties aléatoires complètes pour chaque jeu, IA contre coups aléatoires, audit du point de vue, cohérence du catalogue, réseau en boucle locale).
- Avant une pull request : `cargo fmt --all` et `cargo clippy --release --all-targets -- -D warnings`.
- La CI (`.github/workflows/ci.yml`) exécute fmt, clippy et les tests sous Ubuntu 26.04, puis compile l'interface graphique et lance `polytabletop-gui --selftest`. Une étiquette `v*` déclenche `release.yml`, qui construit des archives pour Linux (x86_64, aarch64), Windows, macOS (arm64, x86_64) avec sommes SHA-256 ; `install.sh` les installe depuis `chiuta/PolyTabletop`. Un `Dockerfile` (CLI et RPG seulement) est fourni.

## Clean-room et marques

- Les règles proviennent uniquement de descriptions publiques des jeux ; aucun code, texte, graphisme ou donnée de produits commerciaux n'est utilisé, et les sources publiques sont notées dans l'en-tête de chaque fichier de jeu.
- La mécanique d'un jeu est libre, son nom, ses illustrations et ses textes ne le sont pas : les jeux portent des noms génériques (« Reversi », « Puissance quatre ») ou, pour les jeux modernes de marque, des variantes originales avec titres, thèmes et contenu propres.
- Aucune marque dans les identifiants, titres, noms de cartes ou banques de contenu. Les noms de marque n'apparaissent que dans le catalogue, comme cibles de planification.
- Les jeux de rôle, de figurines et de cartes à collectionner sont des boîtes à outils génériques (dés, fiches, paquets originaux), jamais une copie des règles ou du contenu d'un éditeur.

## Limites

- L'interface graphique n'a été testée que sans écran (`polytabletop-gui --selftest`) ; elle n'a pas été vérifiée visuellement sur un écran réel.
- La force de l'IA a été mesurée contre des coups aléatoires et entre budgets de temps, pas contre des joueurs humains ; certains jeux ont des limites connues (par exemple le taux de victoire de `macao` et `hruba` est consigné dans le changelog).
- Les règles des jeux écrites de mémoire ont été comparées à des sources publiques, pas à chaque règlement officiel, et les jeux n'ont pas été testés par des joueurs extérieurs.
- Le jeu en réseau n'est testé qu'en boucle locale.
- Les titres et les règles des jeux sont traduits, mais le texte à l'intérieur des jeux (cartes, banques de contenu, invites) est encore en roumain ; les traductions n'ont pas été relues par des locuteurs natifs.
- Les 16 entrées de jeu de rôle sont servies par une boîte à outils générique, pas par des implémentations de ces systèmes.

## Contribuer et historique

Les contributions sont les bienvenues dans le respect des règles clean-room : voir [CONTRIBUTING.md](CONTRIBUTING.md) (ajouter un jeu, tests, liste de contrôle avant pull request). Historique des versions : [CHANGELOG.md](CHANGELOG.md).

## Licence

Déclarée dans `Cargo.toml` comme `MIT OR Apache-2.0` ; textes complets dans [LICENSE-MIT](LICENSE-MIT) et [LICENSE-APACHE](LICENSE-APACHE). Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Crédits

Auteur : Alexio (Alexandru-Ionuț Chiuță), dans la série « Fără Frontiere » (Sans Frontières) d'applications trade-free. Contact : alexio@trom.tf. Page de présentation : <https://polytabletop.chiuta.ro>.
