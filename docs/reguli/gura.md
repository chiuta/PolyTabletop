# Gura Satului (completează fraza) (`gura`)

Gura Satului: joc de completat fraze cu cărți-răspuns pentru doi jucători (205 fraze cu gol și 231 de răspunsuri originale).

## Scop

Fii primul la 6 puncte sau cel mai bun după 12 runde.

## Pregătire

Fiecare are 7 răspunsuri. Judecătorul alternează (J1, J2, J1…). Celălalt joacă un răspuns.

## Desfășurarea unei ture

Celălalt joacă un răspuns (`joaca N`). Pasul forțat `amesteca` adaugă 2 răspunsuri „ale casei” din pachet și amestecă cele 3 cărți fără să arate cine ce a pus. Judecătorul alege cea mai nostimă completare (`alege N`). Pasul forțat `numara` acordă punctul.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `joaca 5` — joci răspunsul 5
* `amesteca` — pas forțat
* `alege 1` — judecătorul alege completarea 1
* `numara` — pas forțat: se acordă punctul

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play gura --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `alege 1` — ca judecător alegi completarea 1.
2. `joaca 7` — în runda următoare joci răspunsul 7.
3. `alege 3` — ca judecător alegi completarea 3.

## Reguli speciale și variante alese

Fără glume cu ținte reale.

## Punctaj

Dacă judecătorul a ales cartea jucătorului, acesta ia un punct; dacă a ales una a casei, ia punctul judecătorul.

## Sfârșitul jocului

Primul la 6 puncte sau cel mai bun după 12 runde.

## Note despre AI și informația ascunsă

AI-ul potrivește etichetele frazei cu cele ale răspunsului (și „hazul” lui); nu vede ce carte a pus cine.

## Diferențe față de regulile tradiționale

Variantă originală a mecanicii genului; frazele și răspunsurile sunt ale noastre.
