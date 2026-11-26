# Puncte și căsuțe (5×5) (`puncte`)

Puncte și căsuțe (5×5): joc cu creion și hârtie în domeniul public, rețea de 6×6 puncte (5×5 căsuțe).

## Scop

Închide mai multe căsuțe decât adversarul.

## Pregătire

Rețea de 6×6 puncte, 60 de linii posibile; coloane a–f, rânduri 1–6.

## Desfășurarea unei ture

Pe rând tragi o linie între două puncte vecine; cine închide a patra latură a unei căsuțe o câștigă și mută din nou (o linie poate închide două căsuțe).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `oa1` — linie orizontală de la punctul a1 spre dreapta
* `va1` — linie verticală de la punctul a1 în sus

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play puncte --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `oe6` — linie orizontală de la e6 spre dreapta.
2. `vc3` — linie verticală de la c3 în sus.
3. `vc5` — linie verticală de la c5 în sus.

## Reguli speciale și variante alese

O mutare care închide o căsuță dă dreptul la o mutare în plus.

## Punctaj

Căsuțe: X (J1) – O (J2).

## Sfârșitul jocului

Când s-au tras toate cele 60 de linii, câștigă cine are mai multe căsuțe (25, deci fără remiză).

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Nu se declară abateri.
