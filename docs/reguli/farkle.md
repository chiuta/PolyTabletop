# Zece mii de puncte (zaruri) (`farkle`)

Joc de zaruri cu titlu propriu, inspirat de tradiționalul Farkle / 10 000 (nume comercial).

## Scop

Primul la 10 000 de puncte (cu rundă finală).

## Pregătire

Șase zaruri.

## Desfășurarea unei ture

`arunc`, apoi păstrezi zarurile care punctează (`pastrez 1 5 5`) și alegi `arunc` cu restul sau `banca`. Fără combinație: Farkle, pierzi punctele turei (`farkle`). Toate cele șase zaruri punctate: le arunci din nou pe toate.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `arunc`
* `pastrez 1 5`
* `banca`
* `farkle`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Combinații de bază: 1 = 100, 5 = 50, trei egale = 100×fața (trei de 1 = 1000), patru = 1000, cinci = 2000, șase = 3000, scară 1500, trei perechi 1500. Cu 0 total banchezi doar de la 500 în tură. Cine atinge 10 000 dă adversarului o ultimă tură. Omise: variantele cu alte combinații.
