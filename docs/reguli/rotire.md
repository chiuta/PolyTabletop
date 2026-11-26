# Rotire în cinci (6×6) (`rotire`)

Rotire în cinci (6×6): tablă din patru cadrane 3×3; pui o bilă, apoi rotești un cadran.

## Scop

Fă cinci bile în linie după rotire.

## Pregătire

Cadrane: 1 stânga-sus, 2 dreapta-sus, 3 stânga-jos, 4 dreapta-jos.

## Desfășurarea unei ture

O tură: pui o bilă pe un pătrat gol, apoi rotești un cadran cu un sfert de tură (`d` = în sensul acelor de ceasornic, `s` = invers).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `c3/1d` — bila pe c3, cadranul 1 spre dreapta
* `c3/1s` — bila pe c3, cadranul 1 spre stânga

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play rotire --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `e2/3s` — bila pe e2, cadranul 3 spre stânga.
2. `c3/2d` — bila pe c3, cadranul 2 spre dreapta.
3. `b2/4d` — bila pe b2, cadranul 4 spre dreapta.

## Reguli speciale și variante alese

Cinci bile în linie (orizontal, vertical, diagonal) după rotire.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Cinci în linie după rotire câștigă; dacă ambii jucători au cinci deodată sau tabla se umple fără cinci, e remiză.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Nu se declară abateri.
