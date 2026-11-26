# Zarurile mincinosului (`liars_dice`)

Liar's Dice (Perudo / Dudo / Cacho), varianta Perudo, pentru doi jucători.

## Scop

Rămâi ultimul cu zaruri.

## Pregătire

Fiecare are 5 zaruri ascunse sub un păhar; la începutul rundei se aruncă în secret.

## Desfășurarea unei ture

Licitezi cantitatea unei fețe pe toate zarurile de pe masă (`3x4`), mereu mai sus, sau spui `dudo`; apoi `arata`. Cine greșește pierde un zar. Asii (fața 1) sunt jokeri pe fețele 2–6.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `3x4`
* `dudo`
* `arata`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Ordinea licitațiilor: mai multă cantitate sau aceeași cantitate cu față mai mare; spre ași se coboară la jumătate (rotunjit în sus), de la ași se urcă la 2×+1. Prima licitație nu poate fi pe ași. Omise: calza și palifico.
