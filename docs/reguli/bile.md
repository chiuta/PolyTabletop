# Bile pe hexagoane (5 pe latură) (`bile`)

Bile pe hexagoane: tablă hexagonală cu 61 de celule, 14 bile pe jucător, mutări prin împingere.

## Scop

Scoate 6 bile adverse.

## Pregătire

Celule: litera rândului (a jos … i sus) + numărul coloanei 1–9.

## Desfășurarea unei ture

Într-o tură muți una, două sau trei bile proprii, vecine pe aceeași linie, cu un pas într-una din cele șase direcții. Mutare „pe lungime”: grupul înaintează pe linia lui și poate împinge 1 sau 2 bile adverse dacă are mai multe bile decât ele (2 contra 1, 3 contra 1 sau 2) și locul de dincolo e liber sau în afara tablei. Mutare „laterală”: grupul se deplasează într-o direcție oblică, toate destinațiile trebuind libere.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `e5>ne` — o bilă de pe e5 spre nord-est
* `c3-c5>se` — grup de la prima (c3) la ultima bilă (c5), spre sud-est
* `direcții` — ne, e, se, sw, w, nw

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play bile --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `a5-b6>sw` — grup a5–b6 spre sud-vest.
2. `a1-a3>w` — grup a1–a3 spre vest.
3. `a1-a2>w` — grup a1–a2 spre vest.

## Reguli speciale și variante alese

Bilele împinse în afară sunt capturate.

## Punctaj

Bile scoase: `X: n bile (a scos k)`.

## Sfârșitul jocului

Câștigă cine scoate 6 bile adverse; remiză după 120 de semi-mutări fără captură (regulă proprie).

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Remiza după 120 de semi-mutări este o regulă proprie; notația este originală.
