# Cărți colecționabile: cerneală, misiuni și provocări (Povestiri din Cerneală) (`tcg_cerneala`)

Cărți colecționabile cu cerneală, misiuni și provocări („Seria Povestirilor din Cerneală”), 2 jucători, set original de 42 de cărți.

## Scop

Fii primul care ajunge la 15 puncte de faimă (sau adversarul rămâne fără pachet la pescuire).

## Pregătire

Pachet pregătit de 30 de cărți (J1 „Pădurea Cântătoare”, J2 „Muntele de Fier”), 7 cărți în mână.

## Desfășurarea unei ture

`pesca` (forțat; J1 nu pescuiește în prima tură), apoi, în orice ordine: `cerneala N` (o dată pe tură, o carte „cu cerneală” din mână se pune întoarsă în călimară; fiecare carte de acolo dă 1 cerneală pe tură), `joaca N [țintă]` (plătești costul în cerneală; personajele intră „uscându-se”), `misiune P` (personajul se înclină și aduce faimă egală cu valoarea lui), `provocare P e Q` (personajul tău pregătit atacă un personaj ÎNCLINAT al adversarului), `fin`. La începutul turei tale totul se pregătește și cerneala se reînnoiește.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pesca` — tragi o carte (forțat)
* `cerneala 3` — pui cartea 3 în călimară
* `joaca 1` — joci cartea 1 (cu țintă opțională `e2`/`m1`)
* `misiune m1` — trimiți personajul m1 în misiune
* `provocare P e Q` — personajul P provoacă personajul advers Q
* `fin` — încheie tura

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tcg_cerneala --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `pesca` — mutarea forțată `pesca` (J1 nu trage carte în prima tură).
2. `cerneala 3` — pui Vrăjitoarea Lacului în călimară (1 cerneală pe tură).
3. `joaca 1` — joci Poetul Rătăcitor (cost 1).

## Reguli speciale și variante alese

Personajele au cost, forță, voință, faimă; daunele rămân până la eliminare (daune ≥ voință). Provocare: ambele lovesc simultan cu forța lor. Cuvinte-cheie: Elan (poate face misiune/provocare imediat), Gardian (dacă adversarul are un Gardian înclinat, provocările trebuie să-l țintească), Zburător (poate fi provocat doar de zburători), Provocator (+2 forță când provoacă), Rezistență (−1 la daunele primite).

## Punctaj

Faimă (0/15), afișată pentru fiecare jucător.

## Sfârșitul jocului

Primul la 15 faimă sau adversarul rămâne fără pachet.

## Note despre AI și informația ascunsă

Mâinile, pachetele și conținutul călimărilor sunt ascunse; `render` arată doar mâna J1. AI-ul (alpha-beta adâncime 1) citește doar câmpul public, mâna proprie și NUMĂRUL cărților adverse.

## Diferențe față de regulile tradiționale

Set original; reguli „scrise de noi”, din mecanici generice.
