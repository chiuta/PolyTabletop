# Hasami Shogi (Japonia) (`hasami`)

Hasami Shogi (Japonia): tablă 9×9, câte 9 piese pe jucător, mutări ca turnul, captură prin încadrare.

## Scop

Lasă adversarul cu cel mult o piesă sau blocat.

## Pregătire

Câte 9 piese pe rândul din spate (X pe rândul 1, O pe rândul 9).

## Desfășurarea unei ture

Orice piesă se mută ca un turn (orice număr de pătrate libere, orizontal sau vertical, fără sărituri). Captură prin încadrare: dacă mutarea ta pune piesa într-o parte a unui șir de piese adverse consecutive, pe un rând sau o coloană, iar la celălalt capăt e o piesă a ta, tot șirul se scoate.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `a1-a5` — muți piesa de la a1 la a5 (fișiere a–i, rânduri 1–9)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play hasami --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `e1-e7` — muți piesa e1 în e7.
2. `c1-c3` — muți piesa c1 în c3.
3. `f1-f3` — muți piesa f1 în f3.

## Reguli speciale și variante alese

Ai voie să intri singur între două piese adverse (nu ești capturat). Captura piesei din colț prin două vecini nu este implementată (colțurile nu au regulă specială).

## Punctaj

Nu există scor.

## Sfârșitul jocului

Câștigă cine lasă adversarul cu cel mult o piesă; cine nu mai poate muta pierde; remiză după 100 de semi-mutări fără captură (regulă proprie).

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Captura piesei din colț prin două vecini nu este implementată; remiza după 100 de semi-mutări este o regulă proprie.
