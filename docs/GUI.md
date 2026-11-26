# Interfața grafică (`polytabletop-gui`)

Fereastră egui/eframe peste aceleași obiecte `GameObj` ca în CLI. Nu face parte din build-ul implicit (necesită biblioteci grafice de sistem: X11 sau Wayland).

```
cargo run --release -p polytabletop-gui              # fereastra
cargo run --release -p polytabletop-gui -- --selftest # verificări fără ecran
```

## Ce oferă

* meniu pe categorii, cu căutare (ex. `sah`, `carti`);
* tablă desenată pentru 16 jocuri abstracte (Șah, Dame, Reversi, Patru în linie, Gomoku, Go, Moara, Xiangqi, Makruk, Janggi, Dara, Morabaraba și familia Mancala: `mancala`, `sungka`, `ayo`, `pallanguzhi`), cu mutări prin click; celelalte jocuri se afișează ca text, cu mutările legale ca butoane și câmp de scris mutarea (buton „Mută”);
* mutări speciale sau forțate ca butoane mari; mutările în mai mulți pași (ex. salturi la dame) se compun prin click, cu „Termină aici” și „Anulează selecția”;
* butonul „Sugestie” (AI-ul calculează pe o copie cu privitorul pus pe cel la mutare, deci nu folosește informația adversarului) și „Joc nou”;
* alegerea cine joacă: Jucătorul 1, Jucătorul 2 sau ambii („la masă”);
* „la masă” cu informație ascunsă (jocurile din `HOTSEAT_SAFE`): după fiecare mutare apare un ecran de predare, iar sugestia e blocată până când jucătorul la mutare preia ecranul; pentru jocurile care ar scurge informația, sugestia e dezactivată;
* salvare și încărcare de jurnal (text); jurnalul se poate relua doar pentru jocurile deterministe, altfel e doar export text;
* temă clară sau întunecată.

## Organizarea codului

| Fișier | Conținut |
|---|---|
| `main.rs` | aplicația, bucla principală, panourile |
| `menu.rs` | lista de jocuri, categorii, căutare |
| `board.rs` | recunoașterea tablei din `render` (`kind_of`), celule, legături |
| `compose.rs` | compunerea mutărilor din click-uri |
| `draw.rs` | desenarea (Painter), teme |
| `session.rs` | logică fără UI: jurnal, replay, convenția „la masă”, permisiuni de sugestie |
| `journal.rs` | formatul de jurnal (cu sumă de control a stării) |
| `selftest.rs` | verificări headless (`--selftest`) |

## Cum adaugi o tablă grafică

Adaugă un `Kind` în `board.rs`, leagă id-ul jocului în `kind_of` și descrie în parser cum se obțin celulele din `render` și denumirile lor din mutări; restul (click, compunere, sugestie) vine din `GameObj`. Jocurile fără tablă grafică funcționează deja în modul text.
