# Focare (cooperativ, boli pe hartă, partener AI) (`focare`)

Focare: joc cooperativ de stingere a bolilor culturilor pe o hartă cu 24 de așezări, cu partener AI. Mecanica este generică (boli pe hartă, pachet de infectare, focare în lanț, epidemii); harta și textele sunt originale.

## Scop

Voi doi (agronomi) vindecați toate cele 4 boli (r = rugina, m = mana, t = tăciunele, f = făinarea) înainte de înfrângere.

## Pregătire

Harta: 24 de așezări (câte 6 din fiecare boală), un inel cu 12 scurtături. Fiecare așezare are 0–3 „spori” din fiecare boală. Tu ești J1; J2 este partenerul AI; mâinile sunt publice. Stația de pornire e în așezarea 1.

## Desfășurarea unei ture

Tura ta: 4 acțiuni, apoi tragi 2 cărți-oraș (mutarea forțată `trage`; două epidemii în pachet fac valuri de infectare), apoi se infectează cărți din pachetul de infectare (`infecteaza`, forțat). Limita mâinii: 7 cărți (`descarta X`). Al 4-lea spor din aceeași boală produce un focar: vecinii primesc câte un spor, în lanț.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `mers X` — mergi într-un vecin
* `zbor X` — renunți la cartea lui X
* `charter X` — renunți la cartea orașului curent
* `naveta X` — între stații
* `statie` — construiești o stație (cartea orașului curent)
* `tratez r` — −1 spor al bolii r (boală vindecată: toți sporii din oraș)
* `dau / iau` — cartea orașului curent, doar dacă sunteți amândoi acolo
* `vindec r` — la stație, 3 cărți de aceeași culoare
* `pas` — treci
* `trage / infecteaza / descarta X / sfârșit` — mutări forțate

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play focare --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `mers bora` — mergi în așezarea Bora.
2. `mers cedra` — mergi în Cedra.
3. `tratez r` — tratezi rugina (−1 spor r) în Cedra.

## Reguli speciale și variante alese

Boală vindecată și fără spori pe hartă = eradicată (nu mai infectează). Mutările forțate sunt separate (`trage`, `infecteaza`) ca AI-ul să nu „vadă” viitorul.

## Punctaj

Afișajul arată boli vindecate (x/4), focare (x/6), rata infectării, mărimile pachetelor și rezerva de spori.

## Sfârșitul jocului

Victorie: toate cele 4 boli vindecate (se afișează „Câștigă Jucătorul 1”). Înfrângere: 6 focare, fără spori în rezervă sau pachetul de cărți golit — se încheie cu REMIZĂ (nu există adversar). Sfârșitul este mutare forțată (`sfârșit`).

## Note despre AI și informația ascunsă

Jocul este cooperativ: J2 este partener AI, iar mâinile sunt publice. Mutările forțate (`trage`, `infecteaza`, `sfârșit`) sunt separate astfel încât AI-ul să nu vadă rezultatul înainte să aleagă.

## Diferențe față de regulile tradiționale

Simplificări declarate față de jocurile de acest tip: fără roluri, fără cărți-eveniment, fără carantină; stația de pornire în așezarea 1; vindecarea folosește doar mâna celui care acționează.
