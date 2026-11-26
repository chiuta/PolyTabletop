# Poliedre (duo, 14×14) (`poliedre`)

Poliedre (duo): tablă 14×14, doi jucători, fiecare cu cele 21 de piese poliomino (1–5 pătrate).

## Scop

Rămâi cu cât mai puține pătrate în piesele nefolosite.

## Pregătire

Piesele se notează cu litere a–u (în ordinea mărimii), orientările cu 1–8: 1–4 rotirile cu câte 90° în sens orar, 5–8 aceleași rotiri ale formei oglindite. Prima piesă a fiecărui jucător acoperă pătratul lui de start (e5 pentru J1, j10 pentru J2).

## Desfășurarea unei ture

Fiecare piesă următoare trebuie să atingă o piesă proprie prin colț și nu are voie să atingă nicio piesă proprie pe latură (piesele adverse pot fi atinse liber). Cine nu mai poate pune trece (`pas`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `k3@e5` — piesa k, orientarea 3, colțul stânga-sus al dreptunghiului piesei la e5
* `pas` — treci

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play poliedre --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `u1@d5` — piesa u (cinci pătrate), orientarea 1, la d5 (acoperă pătratul de start e5).
2. `r3@c8` — piesa r, orientarea 3, la c8.
3. `s4@f2` — piesa s, orientarea 4, la f2.

## Reguli speciale și variante alese

Celula mutării este colțul stânga-sus al dreptunghiului în care încape piesa. Orientările identice cu una anterioară se omit.

## Punctaj

Pătrate rămase în piesele nefolosite (câștigă cine are mai puține).

## Sfârșitul jocului

Partida se termină când ambii trec la rând; egalitate = remiză.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Piesele sunt generate (poliominourile libere); implementare proprie.
