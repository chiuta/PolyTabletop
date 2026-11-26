# Moara (Țintar) (`morris`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play morris --seed 1`. Sursa implementării: `polytabletop-games/src/morris.rs`.

> Rezumat din `polytabletop list`: 9 piese: plasare, apoi mutare pe linii; o moară (trei pe o linie) scoate o piesă adversă; cu 3 piese poți „zbura”. Mutări: d1 (plasare), a1-a4 (mutare); cu moară se adaugă piesa scoasă: g1xb2, a1-a4xb2.

## Scop

Reduci adversarul sub 3 piese sau îl blochezi.

## Pregătire

Tablă cu 24 de puncte; câte 9 piese pe jucător, aflate în afara tablei.

## Desfășurarea turei

Faza 1: pe rând plasezi câte o piesă. Faza 2: muți o piesă pe un punct vecin liber, de-a lungul liniilor. Când ți-au rămas exact 3 piese, poți „zbura” pe orice punct liber.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `d1`.
* Mutare: `a1-a4`.
* Când formezi o moară (trei piese în linie) se adaugă piesa adversă scoasă: `g1xb2` (plasare) sau `a1-a4xb2` (mutare).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (24): `a1`, `d1`, `g1`, `b2`, `d2`, `f2`, `c3`, `d3`, `e3`, `a4` …. Introduci `a1`; răspunsul calculatorului (în ordine): `c4`.
2. Mutări legale (22): `d1`, `g1`, `b2`, `d2`, `f2`, `c3`, `d3`, `e3`, `a4`, `b4` …. Introduci `f4`; răspunsul calculatorului (în ordine): `a4`.
3. Mutări legale (20): `d1`, `g1`, `b2`, `d2`, `f2`, `c3`, `d3`, `e3`, `b4`, `e4` …. Introduci `g7`; răspunsul calculatorului (în ordine): `c3`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Piesa scoasă nu poate face parte dintr-o moară, decât dacă toate piesele adversarului sunt în mori.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Pierde cine rămâne cu mai puțin de 3 piese sau (după plasare) nu are nicio mutare legală. Remiză după 100 de mutări fără moară, odată terminată plasarea.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 10).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 10).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Remiză adăugată: după plasare, 100 de mutări consecutive fără moară.
