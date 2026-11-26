# Gomoku (15×15) (`gomoku`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gomoku --seed 1`. Sursa implementării: `polytabletop-games/src/gomoku.rs`.

> Rezumat din `polytabletop list`: Cinci sau mai multe în linie câștigă; negrul începe în centru. Mutări: h8 (coloane a–o, rânduri 1–15).

## Scop

Aliniezi cinci sau mai multe piese ale tale în linie (orizontal, vertical sau diagonal).

## Pregătire

Tablă 15×15, goală. Negrul (Jucătorul 1) începe; prima mutare este forțată în centru (`h8`, singura mutare legală).

## Desfășurarea turei

Pui o piesă pe o intersecție liberă. Piesele nu se mută și nu se capturează.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Intersecția: `h8` (coloanele a–o, rândurile 1–15).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `h8`. Introduci `h8`; răspunsul calculatorului (în ordine): `g9`.
2. Mutări legale (32): `f6`, `g6`, `h6`, `i6`, `j6`, `e7`, `f7`, `g7`, `h7`, `i7` …. Introduci `h9`; răspunsul calculatorului (în ordine): `h10`.
3. Mutări legale (36): `f6`, `g6`, `h6`, `i6`, `j6`, `e7`, `f7`, `g7`, `h7`, `i7` …. Introduci `g8`; răspunsul calculatorului (în ordine): `f8`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Varianta „free-style”: cinci sau mai multe în linie câștigă (inclusiv șase sau mai multe).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cinci sau mai multe în linie (victorie) sau tabla plină (remiză).

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 8).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 8).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Fără restricțiile de deschidere sau interdicțiile pentru negru din variantele de turneu (renju, swap), adică negrul nu este limitat la 3×3, 4×4 sau linii lungi.
