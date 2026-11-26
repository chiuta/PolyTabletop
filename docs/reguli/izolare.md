# Izolare (7×7) (`izolare`)

Izolare (7×7): fiecare jucător are un pion; muți ca regina și scoți un pătrat de pe tablă.

## Scop

Lasă adversarul fără mutări.

## Pregătire

Pionii: X la d1, O la d7.

## Desfășurarea unei ture

O tură: muți pionul ca regina la șah (orice distanță în linie dreaptă sau diagonală, doar peste pătrate libere), apoi scoți definitiv de pe tablă un pătrat liber (nu cel cu pionul adversarului).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `d1-d3/e5` — muți pionul de la d1 la d3, apoi scoți pătratul e5

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play izolare --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `d1-d5/e7` — pion d1→d5, scoți e7.
2. `d5-e4/c4` — pion d5→e4, scoți c4.
3. `e4-e5/d2` — pion e4→e5, scoți d2.

## Reguli speciale și variante alese

Pătratele scoase apar marcate cu `#`.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Cine nu mai poate muta la începutul turei pierde; nu există remiză.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Nu se declară abateri.
