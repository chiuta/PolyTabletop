# Hex (11×11) (`hex`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hex --seed 1`. Sursa implementării: `polytabletop-games/src/hex.rs`.

> Rezumat din `polytabletop list`: X unește sus și jos, O unește stânga și dreapta printr-un lanț de hexagoane vecine; nu există remiză, X începe. Mutări: f6 (coloane a–k, rânduri 1–11).

## Scop

X (Jucătorul 1) unește marginea de sus cu cea de jos (rândurile 1 și 11); O (Jucătorul 2) unește marginea stângă cu cea dreaptă (coloanele a și k), printr-un lanț de hexagoane vecine.

## Pregătire

Romb de 11×11 hexagoane, gol. X începe.

## Desfășurarea turei

Ocupi un hexagon liber cu piesa ta. Piesele nu se mută.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Hexagonul: `f6` (coloanele a–k, rândurile 1–11).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (121): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `g1`, `h1`, `i1`, `j1` …. Introduci `f6`; răspunsul calculatorului (în ordine): `a2`.
2. Mutări legale (119): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `g1`, `h1`, `i1`, `j1` …. Introduci `e7`; răspunsul calculatorului (în ordine): `b2`.
3. Mutări legale (117): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `g1`, `h1`, `i1`, `j1` …. Introduci `k11`; răspunsul calculatorului (în ordine): `j1`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Fără regula de schimb („pie rule”).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Când cineva închide lanțul între laturile sale; nu există remiză.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (600 ms, adâncime maximă 4).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (600 ms, adâncime maximă 4).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Lipsește regula de schimb, care în jocul de turneu compensează avantajul primului jucător.
