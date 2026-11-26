# Gospodăria (plasare de muncitori și fermă) (`gospodaria`)

Gospodăria: joc de plasare de muncitori și gospodărie pentru doi jucători, 6 runde, informație completă (seed-ul stabilește doar cine începe).

## Scop

Ai mai multe puncte la final.

## Pregătire

Familia începe cu 2 membri și 2 camere; fiecare membru e un muncitor pe rundă. Ferma e doar contoare (camere, câmpuri, țarcuri).

## Desfășurarea unei ture

Pe rând plasezi un muncitor pe o acțiune liberă (`pune lemn`); fiecare acțiune ține un singur muncitor pe rundă. Acțiunile cu „+” se acumulează cât timp nu sunt luate: `lemn` (+3), `lut` (+2), `peste` (+1 hrană), `oi` (+1 oaie; oile fără loc devin hrană 1:1), `cereale`, `legume`, `zilier` (+2 hrană), `arat` (câmp nou, cel mult 5), `semanat n`, `camera` (3 lemn + 2 lut, cel mult 4 camere), `tarc` (2 lemn, cel mult 3), `familie`, `inceput` (1 hrană și începi runda următoare).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pune lemn` — muncitor pe lemn (și lut, peste, oi, cereale, legume, zilier, arat, camera, tarc, familie)
* `pune semanat 2` — semeni 2 câmpuri
* `pune inceput` — devii primul jucător în runda următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play gospodaria --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `pune inceput` — muncitor pe `inceput` (+1 hrană, începi runda următoare).
2. `pune oi` — muncitor pe `oi`.
3. `pune inceput` — din nou `pune inceput` (în runda următoare, după recoltă).

## Reguli speciale și variante alese

Recoltă după rundele 2, 4 și 6: fiecare câmp cu cereale dă 1 cereale. Hrănire: 2 hrană pe membru; se consumă automat hrana, apoi cereale (1), oi (2) și legume (2); fiecare hrană lipsă = o cerșetorie (−3 puncte). Apoi fiecare pereche de oi cu loc liber face un miel (loc: 2 pe țarc + 1 în casă). Semănat: fiecare câmp gol semănat ia 1 cereale și primește 3.

## Punctaj

Punctaj final: câmpuri, țarcuri, cereale, oi, legume (0 = −1; 1–4 → 1–4 puncte, 4+ = 4 puncte; oi 1–3=1, 4–5=2, 6–7=3, 8+=4; cereale la fel), camere peste 2: +1 fiecare, +3 pe membru de familie, −3 pe cerșetorie.

## Sfârșitul jocului

După runda 6.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul (alpha-beta adâncime 7) vede toată starea.

## Diferențe față de regulile tradiționale

Simplificări declarate: ferma nu e o grilă, ci contoare; o singură cultură semănată (cereale) și o singură specie (oi); fără cărți.
