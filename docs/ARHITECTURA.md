# Arhitectura PolyTabletop

## Structura workspace-ului

| Crate | Rol |
|---|---|
| `polytabletop-core` | trait-ul tipat `Game`, trait-ul obiect-sigur `GameObj` (mutări ca text), `Player`, `Outcome`, `AiStyle`, `rng` (generator propriu) și `ai` (alpha-beta, alpha-beta cu timp, MCTS/UCT). Fără dependențe externe. |
| `polytabletop-games` | toate jocurile (un modul pe joc), `playable()` (lista jocurilor jucabile), `HOTSEAT_SAFE`, `Kind`/`E` și catalogul generat `catalog.rs` (251 de intrări; 171 de jocuri jucabile). Fără dependențe externe. |
| `polytabletop` | interfața în linie de comandă: `polytabletop list`, `polytabletop catalog`, `polytabletop play`. |
| `polytabletop-gui` | interfața grafică egui/eframe (singurul crate cu dependențe externe); în afara build-ului implicit. |
| `polytabletop-rpg` | trusă generică pentru jocuri de rol (zaruri, fișe, teste, dungeon); vezi `polytabletop-rpg/README.md`. |

Unelte: `tools/gen_catalog.py` (regenerează `catalog.rs` din `tools/data`) și `tools/gen_docs.py` (regenerează `docs/JOCURI.md` și `docs/CATALOG.md` din binarul `polytabletop`).

## Trait-ul `Game`

Un joc implementează `polytabletop_core::Game` (Clone + tip `Move`):

* obligatorii: `to_move`, `legal_moves` (goală doar când jocul s-a terminat), `apply`, `outcome` (`Win(Player)` sau `Draw`), `render` (text), `format_move` (textul canonic al mutării; omul scrie exact acest text, fără majuscule/spații semnificative);
* recomandate: `evaluate(p)` (euristică din punctul de vedere al lui `p`), `ai_style`;
* opționale pentru căutare: `hash_key` (tabel de transpoziție), `move_score`, `move_key` (history heuristic), `quiescence`/`is_tactical`, `playout_move`/`playout_cap` (MCTS);
* `set_viewer(Option<Player>)`: vezi mai jos.

O implementare generală transformă orice `Game` în `GameObj` (`render`, `to_move`, `legal_moves` ca `Vec<String>`, `play(&str)`, `outcome`, `ai_move(seed)`, `clone_box`, `set_viewer`). Interfețele lucrează doar cu `GameObj`, deci un joc nou apare gratis în CLI și GUI. Textele mutărilor dintr-o poziție trebuie să fie unice (testat).

## Cum adaugi un joc

1. Creează `polytabletop-games/src/<id>.rs` cu structura jocului și `impl Game`; pune în antet sursele publice ale regulilor.
2. Declară `pub mod <id>;` în `polytabletop-games/src/lib.rs`.
3. Adaugă o intrare `Playable { id, title, category, rules, make }` în `playable()`. Câmpul `rules` conține regulile pe scurt și, după `Mutări:`, sintaxa cu exemple (de aici se generează `docs/JOCURI.md`).
4. Dacă jocul ascunde informație privată și `render` respectă `set_viewer`, adaugă id-ul în `HOTSEAT_SAFE`.
5. Dacă are intrare în catalog, leagă-o în `tools/data` (câmpul `impl_id`) și rulează `python3 -I tools/gen_catalog.py`.
6. Adaugă teste (vezi mai jos), apoi `cargo fmt`, `cargo clippy -D warnings`, `python3 -I tools/gen_docs.py`.

## Stiluri de AI (`AiStyle`)

* `AlphaBeta(adâncime)`: alpha-beta cu adâncime fixă pe `evaluate` (implicit 4).
* `Mcts(iterații)`: UCT Monte-Carlo cu simulări prin `playout_move` (jocuri cu zaruri, cărți, factor de ramificare mare).
* `AlphaBetaTime { ms, max_depth }`: adâncire iterativă cu buget de timp, ordonarea mutărilor (mutarea precedentă, killers, history, `move_score`), tabel de transpoziție dacă există `hash_key`, quiescence dacă `quiescence()` e adevărat. Bugetul se poate suprascrie cu variabila `POLYTABLETOP_AI_MS` sau `ai::set_time_budget_override_ms`.

Regula de fier: AI-ul nu trișează. `evaluate` și simulările citesc doar informația vizibilă jucătorului pentru care se caută; jocurile cu mâini ascunse determinizează (amestecă aleator partea necunoscută) în loc să citească mâna adversarului.

## Convenția „la masă” (hot-seat) și `set_viewer`

`set_viewer(Some(p))` fixează privitorul: `render` arată mereu informația privată a lui `p`. `set_viewer(None)` înseamnă „la masă”: `render` arată informația celui la mutare. CLI-ul pune `Some(Jucător 1)` implicit, `Some(Jucător 2)` la `--you two` și `None` la `--you both`.

`HOTSEAT_SAFE` listează jocurile al căror `render` respectă convenția. Pentru jocurile cu stare aleatoare care nu sunt în listă, `polytabletop play --you both` afișează un avertisment, iar `hint` este dezactivat (sugestia ar folosi starea completă). Zarurile singure (categoriile Abstract, Clasic, Zaruri) nu se consideră informație privată. În GUI, după fiecare mutare în „la masă” cu informație ascunsă apare un ecran de predare.

## Testare

`cargo test --release` (profilul de test e compilat cu `opt-level = 2`). Teste în `polytabletop-games/tests/`:

* `all_games.rs`: verificări peste toate jocurile jucabile (partide aleatorii complete, mutări unice, coerența catalogului);
* `board_games.rs`, `cardsb_games.rs`, `hidden_games.rs`: reguli pe familii (perft la șah 20/400/8 902/197 281, rocadă, en passant, ko, captură obligatorie);
* `ai_match.rs`, `ai_strength.rs`, `y4_ai.rs`: AI contra mutări aleatorii și puterea AI-ului;
* `y2_viewer.rs`, `y3_viewer.rs`: `set_viewer` (hot-seat egal cu privitorul fixat pe cel la mutare, fără scurgeri);
* teste pe valuri (`w2_*`..`w5_*`, `x*`, `y5_*`) pentru jocurile adăugate pe rând.

Fiecare agent nou își adaugă un fișier propriu `polytabletop-games/tests/<nume>_*.rs`. Pentru build-uri rapide: `CARGO_PROFILE_RELEASE_LTO=false CARGO_INCREMENTAL=0` și un `CARGO_TARGET_DIR` separat.

## Politica clean-room

* Regulile vin doar din descrieri publice; nu se folosește cod, text, grafică sau date ale produselor comerciale. Sursele se notează în antetul fiecărui fișier de joc.
* Mecanica nu e protejată, dar numele, grafica și textele da: jocurile poartă nume generice (Reversi, Patru în linie), iar jocurile moderne cu marcă sunt variante originale cu nume și conținut proprii (cărți, bănci de cuvinte, texte scrise de noi).
* Jocurile de rol, cu miniaturi și cărțile colecționabile se fac ca toolkit generic (`polytabletop-rpg`, motoare TCG cu cărți originale), nu prin copierea regulilor sau conținutului unui editor.
* Numele mărcilor nu apar în `id`-urile, titlurile sau textele jocurilor noastre; apar doar în catalog, ca țintă de planificare.
