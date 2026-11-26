# Fulger (simbolul comun, cronometru simulat) (`fulger`)

Fulger: potrivire de simboluri pe cărți, cu 31 de cărți, 6 simboluri pe carte; oricare două cărți au exact un simbol comun.

## Scop

Numește primul (și corect) simbolul comun dintre cartea ta și cea din mijloc; ai mai multe puncte când pachetul se golește.

## Pregătire

Fiecare jucător are o carte în față, la mijloc stă o carte. Cronometrul este simulat pe ture: omul și masa primesc fiecare un timp de reacție tras la zar (masa devine puțin mai rapidă pe parcurs).

## Desfășurarea unei ture

Numești simbolul comun scriind numele lui (ex. `luna`). Cine îl numește corect și mai repede ia cartea din mijloc (devine cartea lui) și un punct. Un simbol greșit dă runda adversarului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `luna` — numele simbolului comun (ex. `piatra`, `ancora`)
* `cronometru` — afișează cronometrul

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play fulger --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `tigaie` — numești `tigaie` (greșit în acest seed: runda e a masei).
2. `ancora` — numești `ancora` (corect).
3. `scara` — numești `scara` (greșit).

## Reguli speciale și variante alese

Masa greșește uneori.

## Punctaj

Un punct pentru cine ia cartea din mijloc.

## Sfârșitul jocului

Pachetul gol: câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

Masa vede doar cartea ei și pe cea din mijloc, nu cartea ta. Timpii de reacție sunt trași la zar.

## Diferențe față de regulile tradiționale

Variantă originală de „găsește simbolul comun”; cronometru simulat în loc de viteză reală.
