# Crokinole (`crokinole`)

Crokinole, joc de dexteritate modelat ca simulare fizică deterministă, pentru doi jucători.

## Scop

Primul la 100 de puncte (cel mult 6 runde).

## Pregătire

Tablă circulară cu gaura centrală de 20, inele de 15/10/5 și opt cuie; 12 discuri fiecăruia, trase de pe linia proprie.

## Desfășurarea unei ture

Alegi poziția, unghiul și puterea (`x-1 u6 p3`), apoi `lasa` execută lovitura cu zgomot din seed. Dacă pe tablă e un disc advers, trebuie să-l lovești direct; altfel discul tras iese din joc.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `x-1 u6 p3`
* `lasa`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Geometria e aproximativă, nu cea oficială. Lovitura se alege în doi pași. După 24 de lovituri câștigătorul rundei ia diferența de puncte. AI-ul nu vede zgomotul real: evaluează țintirile pe simulări cu zgomot propriu.
