# Valea Uscată (duel western, roluri ascunse) (`valea`)

Valea Uscată: duel western cu roluri ascunse pentru 5 pistolari. Tu ești locul 1; locurile 2–5 sunt roboți.

## Scop

Șerif și Adjunct: mor toți haiducii și trădătorul. Haiduci: moare Șeriful. Trădător: rămâne ultimul în viață.

## Pregătire

Roluri: un Șerif (public), un Adjunct, doi Haiduci și un Trădător. Viață: 5 (Șerif) sau 4. Cărți: Foc, Ocolire, Leac, Salon, Mitralieră, Pușcă, Panică.

## Desfășurarea unei ture

Pe tură tragi 2 cărți, joci câte vrei, apoi arunci până la câte vieți ai (`scarta foc`). Foc: o dată pe tură, la distanță 1 (2 cu Pușcă). Când ești țintit alegi `ocolire` sau `accept`. `incheie` termină tura. `polytabletop` lasă roboții să acționeze.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `foc 3` — tragi în locul 3
* `leac` — +1 viață
* `salon` — +1 viață pentru toți
* `mitraliera` — Foc la toți ceilalți
* `pusca` — echipezi pușca (bătaie 2)
* `panica 2` — iei o carte la întâmplare de la locul 2
* `incheie` — încheie jucatul cărților
* `scarta foc` — arunci o carte
* `ocolire / accept` — răspunsul când ești țintit
* `polytabletop` — roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play valea --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `foc 5` — tragi în locul 5.
2. `incheie` — încheie cărțile jucate în tura asta.
3. `scarta foc` — arunci o carte `foc` pentru a ajunge la limita vieților.

## Reguli speciale și variante alese

Cine omoară un haiduc trage 3 cărți; Șeriful care omoară Adjunctul aruncă tot. Secetă: din runda 30, fiecare pierde 1 viață la începutul turei lui (garantează finalul). Ocolire anulează un Foc/Mitralieră asupra ta.

## Punctaj

Nu există scor, doar eliminări.

## Sfârșitul jocului

Vezi condițiile din scop.

## Note despre AI și informația ascunsă

Roboții își deduc aliații doar din fapte publice (cine a tras în Șerif). Rolul tău e afișat doar ție.

## Diferențe față de regulile tradiționale

Joc original, nu o replică a unui joc existent; fără abateri declarate.
