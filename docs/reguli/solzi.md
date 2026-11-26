# Solzi (variantă originală) (`solzi`)

Solzi: motor de cărți-animale (reptile și amfibieni) cu habitate, hrană și ouă, variantă originală pentru doi.

## Scop

Ai scorul mai mare după 4 runde (puncte de pe cărți + ouă + obiective).

## Pregătire

Fiecare jucător are un teren cu 3 habitate (Pădure P, Stâncă S, Baltă B), câte 5 locuri; 4 runde cu 8, 7, 6, 5 acțiuni. Hrana: insecte (I), viermi (V), pește (F), șoareci (M).

## Desfășurarea unei ture

O acțiune pe tură: `joc 2 b` — joci cartea 2 din mână în habitatul B (dacă cartea trăiește acolo și mai e loc): plătești hrana din colțul cărții și ouă după coloană (0, 1, 1, 2, 2 ouă). `padure I V` — Pădurea dă hrană: 1 + (vietăți în Pădure)/2 unități, alese de tine; apoi fiecare vietate cu putere „hrană” dă încă o unitate. `stanca p1 s2` — Stânca dă ouă: 1 + (vietăți în Stâncă)/2 ouă pe locurile alese; apoi vietățile cu putere „ou” pun un ou pe ele. `balta` — Balta trage 1 + (vietăți în Baltă)/2 cărți; apoi puterile „carte” trag câte una.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `joc 4 s` — joci cartea 4 în Stâncă
* `padure I V` — iei hrană: insectă și vierme
* `stanca p1 s2` — pui ouă pe vietățile p1 și s2
* `balta` — tragi cărți

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play solzi --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `joc 4 s` — joci Broasca roșie în Stâncă.
2. `joc 3 p` — joci Țestoasa mlăștinoasă în Pădure.
3. `joc 1 b` — joci Broasca râioasă în Baltă.

## Reguli speciale și variante alese

Puterile se declanșează doar dacă vietatea stă în habitatul acțiunii (o vietate cu putere de hrană pusă în Baltă nu face nimic). Ouăle se iau de pe vietățile cu cele mai multe ouă.

## Punctaj

La sfârșitul fiecărei runde: obiectiv comun (rundele 1–4: cele mai multe ouă / vietăți / puncte de pe cărți / hrană în rezervă) = 4, 5, 6, 7 puncte pentru câștigător (egal: jumătate fiecare). Scor final: punctele cărților, ouăle, obiectivele.

## Sfârșitul jocului

După runda 4.

## Note despre AI și informația ascunsă

Informație ascunsă: pachetul și mâna adversarului; `render` arată doar mâna J1. AI-ul numără cărțile din mână, nu identitatea lor.

## Diferențe față de regulile tradiționale

Joc original; cărțile sunt generate.
