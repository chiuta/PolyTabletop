# Morabaraba (Africa de Sud) (`morabaraba`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play morabaraba --seed 1`. Sursa implementării: `polytabletop-games/src/morabaraba.rs`.

> Rezumat din `polytabletop list`: Tablă ca la Moară plus diagonalele prin colțuri (20 de „vaci”), 12 piese. Plasare, apoi mutare pe linii (și diagonale); vaca scoate o piesă adversă (nu din vacă, dacă are alta); cu 3 piese zbori; sub 3 piese sau blocat = pierdere; tablă plină după plasare = remiză. Mutări: d1, a1-b2, c3xg7, a1-d5xg7.

## Scop

Reduci adversarul la 2 piese sau îl blochezi.

## Pregătire

24 de puncte ca la Moară plus diagonalele prin colțuri (a1-b2-c3, g1-f2-e3, a7-b6-c5, g7-f6-e5): 20 de „vaci” (linii de trei). 12 piese pe jucător.

## Desfășurarea turei

Faza 1: plasare, câte o piesă pe rând. Faza 2: muți o piesă pe un punct vecin liber, inclusiv diagonale. Cu exact 3 piese poți „zbura”.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `d1`.
* Mutare: `a1-b2`.
* Cu vacă formată, se adaugă piesa scoasă: `c3xg7` (plasare), `a1-d5xg7` (mutare).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (24): `a1`, `d1`, `g1`, `b2`, `d2`, `f2`, `c3`, `d3`, `e3`, `a4` …. Introduci `a1`; răspunsul calculatorului (în ordine): `d7`.
2. Mutări legale (22): `d1`, `g1`, `b2`, `d2`, `f2`, `c3`, `d3`, `e3`, `a4`, `b4` …. Introduci `e4`; răspunsul calculatorului (în ordine): `d5`.
3. Mutări legale (20): `d1`, `g1`, `b2`, `d2`, `f2`, `c3`, `d3`, `e3`, `a4`, `b4` …. Introduci `g7`; răspunsul calculatorului (în ordine): `e5`.

## Reguli speciale și variante alese

* O vacă formată scoate o piesă adversă, nu dintr-o vacă, decât dacă toate sunt în vaci.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Pierde cine rămâne cu 2 piese sau nu mai poate muta. Remiză dacă tabla se umple complet la sfârșitul plasării, sau după 100 de mutări fără captură.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă (4, respectiv 5).

Stil AI în cod: alpha-beta cu adâncime fixă 4; alpha-beta cu adâncime fixă 5.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* În sursă, remiza vine când un jucător are 3 vaci și nimeni nu capturează în 10 mutări; aici sunt 100 de mutări fără captură.
* La regulile Mind Sports South Africa, o vacă desfăcută nu se poate reface la mutarea următoare (neimplementat).
