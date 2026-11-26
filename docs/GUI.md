# Interfața grafică (`polytabletop-gui`)

Fereastră egui/eframe peste aceleași obiecte `GameObj` ca în CLI. Nu face parte din build-ul implicit (necesită biblioteci grafice de sistem: X11 sau Wayland).

```
cargo run --release -p polytabletop-gui              # fereastra
cargo run --release -p polytabletop-gui -- --selftest # verificări fără ecran
```

## Ce oferă

* meniu pe categorii, cu căutare (ex. `sah`, `carti`);
* tablă desenată pentru 36 de jocuri (Șah, Dame, Reversi, Patru în linie, Gomoku, Go, Moara, Xiangqi, Makruk, Janggi, Dara, Morabaraba, Shogi, Table, Nu te supăra frate, Pachisi, Hex, Amazoane, Linii de acțiune, Konane, Fanorona, Tigri și capre, Surakarta, Hasami Shogi, Puncte și căsuțe, Străpungere, Izolare, Rotire în cinci, Coridor, Drum de pietre, Trei-pierde, Nim și familia Mancala: `mancala`, `sungka`, `ayo`, `pallanguzhi`; lista completă în `kind_of`, `board.rs`), cu mutări prin click; celelalte jocuri se afișează ca text, cu mutările legale ca butoane și câmp de scris mutarea (buton „Mută”);
* mutări speciale sau forțate ca butoane mari; mutările în mai mulți pași (ex. salturi la dame) se compun prin click, cu „Termină aici” și „Anulează selecția”;
* butonul „Sugestie” (AI-ul calculează pe o copie cu privitorul pus pe cel la mutare, deci nu folosește informația adversarului) și „Joc nou”;
* alegerea cine joacă: Jucătorul 1, Jucătorul 2 sau ambii („la masă”);
* „la masă” cu informație ascunsă (jocurile din `HOTSEAT_SAFE`): după fiecare mutare apare un ecran de predare, iar sugestia e blocată până când jucătorul la mutare preia ecranul; pentru jocurile care ar scurge informația, sugestia e dezactivată;
* salvare și încărcare de jurnal (text); jurnalul se poate relua doar pentru jocurile deterministe, altfel e doar export text;
* temă „Fără Frontiere” clară sau întunecată, cu fonturi încorporate (Cormorant Garamond, Lora, JetBrains Mono, DejaVu Sans pentru simboluri), fără rețea la rulare; limba se păstrează în `gui.conf`;
* cărți de joc (pachet francez, italian, spaniol, tarot), zaruri și dominouri desenate vectorial (`cards.rs`, `cards_ui.rs`) pentru jocurile de cărți, zaruri și domino recunoscute; textul `render()` rămâne vizibil dedesubt, iar jocurile nerecunoscute păstrează butoanele text.

## Organizarea codului

| Fișier | Conținut |
|---|---|
| `main.rs` | aplicația, bucla principală, panourile |
| `menu.rs` | lista de jocuri, categorii, căutare |
| `board.rs` | recunoașterea tablei din `render` (`kind_of`), celule, legături |
| `compose.rs` | compunerea mutărilor din click-uri |
| `draw.rs` | desenarea (Painter), teme |
| `theme.rs` | tema „Fără Frontiere” (paletă clară/întunecată) și fonturile încorporate |
| `cards.rs`, `cards_ui.rs` | cărți, zaruri și dominouri desenate vectorial; parserul de jetoane din `render()`; integrarea în sesiune |
| `settings.rs`, `gui_i18n.rs` | setări persistente (limba) și traducerile interfeței (8 limbi) |
| `markers.rs` | marcatorii din `render()` după care se decupează tablele |
| `session.rs` | logică fără UI: jurnal, replay, convenția „la masă”, permisiuni de sugestie |
| `journal.rs` | formatul de jurnal (cu sumă de control a stării) |
| `selftest.rs` | verificări headless (`--selftest`) |

## Cum adaugi o tablă grafică

Adaugă un `Kind` în `board.rs`, leagă id-ul jocului în `kind_of` și descrie în parser cum se obțin celulele din `render` și denumirile lor din mutări; restul (click, compunere, sugestie) vine din `GameObj`. Jocurile fără tablă grafică funcționează deja în modul text.
