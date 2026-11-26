# Tigri și capre (Nepal) (`tigri`)

Tigri și capre (Nepal, „Bagh-Chal”): tablă 5×5 de puncte; tu ești caprele, calculatorul are tigrii.

## Scop

Tigrii: 5 capre capturate. Caprele: imobilizează toți tigrii.

## Pregătire

Tablă 5×5 de puncte, linii ortogonale plus diagonale prin punctele „pare”. Patru tigri (J2) pornesc în colțuri; 20 de capre (J1, mută primul) sunt în mână.

## Desfășurarea unei ture

Caprele se plasează pe rând, câte una pe orice punct liber, apoi (după ce au fost plasate toate) se mută cu un pas pe un punct vecin liber. Tigrii se mută cu un pas sau sar peste o capră vecină, pe aceeași linie, pe punctul liber de după ea, capturând-o (captura nu e obligatorie).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `c3` — plasezi o capră pe c3
* `a1-b2` — pas de la a1 la b2
* `a1xc1` — săritură cu captură de la a1 la c1

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tigri --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `a3` — plasezi o capră pe a3.
2. `e3` — plasezi o capră pe e3.
3. `c1` — plasezi o capră pe c1.

## Reguli speciale și variante alese

Captura nu e obligatorie.

## Punctaj

Capre capturate: x/5.

## Sfârșitul jocului

Tigrii câștigă cu 5 capre capturate; caprele câștigă dacă toți tigrii sunt imobilizați. Remiză după 100 de semi-mutări fără captură în faza de mutare.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Remiza după 100 de semi-mutări fără captură în faza de mutare este o regulă proprie, în loc de repetiția pozițiilor.
