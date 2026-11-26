# Surakarta (Java) (`surakarta`)

Surakarta (Java): tablă 6×6, 12 piese pe jucător, capturi pe circuite cu arcuri de colț.

## Scop

Scoate toate piesele adverse.

## Pregătire

Piesele ocupă rândurile 1–2 (X) și 5–6 (O). Pe tablă există două circuite închise: cel mic (liniile 2 și 5) și cel mare (liniile 3 și 4), fiecare legat în cele patru colțuri prin arcuri.

## Desfășurarea unei ture

O mutare simplă merge pe un punct vecin liber (și pe diagonală). O captură merge pe un circuit, în oricare sens, peste puncte libere, trecând prin cel puțin un arc de colț, și se oprește pe prima piesă întâlnită, dacă e a adversarului, pe care o scoate.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `b2-b3` — mutare simplă
* `a2xb6` — captură prin circuit: piesa de pe a2 ia piesa de pe b6

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play surakarta --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `f2-f3` — mutare simplă f2-f3.
2. `d2-c3` — mutare simplă d2-c3.
3. `e1-d2` — mutare simplă e1-d2.

## Reguli speciale și variante alese

Piesele proprii blochează; punctul de plecare al piesei contează ca liber la reîntoarcere.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Câștigă cine rămâne singurul cu piese; remiză după 60 de semi-mutări fără captură (regulă proprie).

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Remiza după 60 de semi-mutări fără captură este o regulă proprie.
