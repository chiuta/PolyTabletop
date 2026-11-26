# Străjile Codrului (cooperativ de apărare, cărți de putere) (`strajile`)

Străjile Codrului: joc cooperativ de apărare cu cărți de putere, în care doi duhuri (J1 Rădăcinilor, J2 Furtunii, AI) apără o vale cu 8 ținuturi de invadatori.

## Scop

Provocați frică 28 sau curățați valea de invadatori după ultima carte de invadatori, evitând 10 blesteme.

## Pregătire

Harta: 8 ținuturi (M munte, P pădure, B mlaștină, C câmpie; 4 de coastă), un inel cu 3 scurtături. Pe ținuturi stau invadatori (pionieri forță 1, colonii 2, fortărețe 3), localnici, blestem și prezența duhurilor. Pachet de 12 cărți de invadatori în trei trepte (explorare → construire → devastare), durata de 14 runde.

## Desfășurarea unei ture

(1) Creștere, o singură alegere: `creste N` (prezență într-un ținut vecin, +1 energie), `aduna` (+2 energie) sau `reia` (cărțile jucate revin în mână, +1 energie); (2) cel mult două cărți de putere: `joc spini 3` (plătești costul; ținutul trebuie să fie în raza cărții față de prezența ta); (3) `gata`. Apoi runda invadatorilor, în trei mutări forțate: `devastare`, `constructie`, `explorare`; cărțile avansează un slot.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `creste 7` — adaugi prezență în ținutul 7
* `aduna` — +2 energie
* `reia` — cărțile jucate revin în mână
* `joc spini 3` — joci cartea „spini” asupra ținutului 3
* `gata` — termini tura
* `devastare / constructie / explorare` — mutări forțate ale invadatorilor
* `sfârșit` — mutare forțată de încheiere

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play strajile --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `creste 7` — extinzi prezența în ținutul 7 (Piscuri).
2. `joc furtuna 7` — joci cartea „furtuna” asupra ținutului 7.
3. `gata` — termini tura.

## Reguli speciale și variante alese

Construire: pionierii devin colonii; o colonie existentă aduce un pionier nou și (în teroare 1) o fortăreață. Devastare: forța invadatorilor minus apărarea ținutului; fiecare 2 puncte ucid un localnic; restul ≥ 1 aduce un blestem. Frică ≥ 14: „teroare 2” — invadatorii nu mai construiesc fortărețe. Explorare: pe țărm sau lângă invadatori apar 3 pionieri (4 în a doua jumătate a pachetului).

## Punctaj

Frică: fiecare colonie ucisă dă 1, fortăreață 2 (+ efectele cărților).

## Sfârșitul jocului

Echipa câștigă la frică 28 sau dacă valea e curată de invadatori după ultima carte (J1 „câștigă”). 10 blesteme sau invadatori rămași la sfârșit = înfrângere = REMIZĂ.

## Note despre AI și informația ascunsă

Cooperativ; partenerul J2 este AI. Mutările invadatorilor sunt forțate, deci nu pot fi anticipate; mâna ambilor duhuri e vizibilă în afișaj.

## Diferențe față de regulile tradiționale

Joc original (mecanică generică); nu se declară alte abateri.
