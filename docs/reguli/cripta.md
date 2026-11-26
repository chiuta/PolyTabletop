# Cripta Clopotelor (aventură tactică cu cărți de acțiune) (`cripta`)

Cripta Clopotelor: aventură cooperativă tactică cu cărți de acțiune; Paznicul de Far (J1) și Vânătoarea de Umbre (J2, AI) coboară într-o criptă.

## Scop

Îndepliniți obiectivul scenariului: (1) omorâți toți monștrii, (2) ajungeți la ieșire, (3) omorâți Regele Clopot și garda.

## Pregătire

Trei scenarii originale („Subsolul Clopotarului”, „Coridorul Oglinzilor”, „Tronul Roșu”), alese la întâmplare la început. Fiecare erou are o mână de 8 cărți cu inițiativă și o jumătate de sus și una de jos.

## Desfășurarea unei ture

Fiecare erou joacă `joc 3 5` (cartea 3 conduce: inițiativa ei; a doua e 5) sau `odihna` (inițiativă 99: +2 viață, cărțile jucate revin în mână, dar una la întâmplare se pierde; cere cel puțin 2 cărți jucate). Apoi `revela` (forțat): monștrii trag cărți de comportament și ordinea se stabilește după inițiativă. Tura eroului: `sus 3 jos 5` sau `sus 5 jos 3`; apoi fiecare acțiune: `mers c4`, `atac m2` / `atac toti`, `aplic`, `sari`. Atacul își trage modificatorul într-o mutare forțată (`trage`). Monștrii acționează la `monstri` (forțat).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `joc 6 5` — alegi cărțile 6 (conduce) și 5
* `odihna` — odihnă
* `revela` — dezvăluie și stabilește ordinea (forțat)
* `sus 3 jos 5` — joci partea de sus a cărții 3 și cea de jos a cărții 5
* `mers c4` — muți la c4
* `atac m2 / atac toti` — ataci
* `aplic` — vindecare/scut
* `sari` — sari acțiunea
* `trage / monstri / sfârșit` — mutări forțate

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play cripta --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `joc 6 5` — alegi cărțile 6 (Cârpeli, init 80) și 5 (Ecoul Clopotului).
2. `monstri` — `monstri` — mutare forțată: monștrii acționează.
3. `monstri` — încă o mutare forțată `monstri`.

## Reguli speciale și variante alese

Scutul reduce fiecare lovitură primită în runda aceea. Un erou rămâne fără viață sau fără cărți (mai puțin de 2 în mână și 2 jucate) = epuizat. La egalitate de inițiativă, eroii înaintea monștrilor.

## Punctaj

Afișajul arată viața eroilor, cărțile în mână, jucate și pierdute.

## Sfârșitul jocului

Obiectivul îndeplinit = victorie (J1 „câștigă”); ambii eroi epuizați sau 25 de runde = înfrângere = REMIZĂ.

## Note despre AI și informația ascunsă

Cooperativ; J2 este AI. Mutările de modificator de atac și comportamentul monștrilor sunt mutări forțate.

## Diferențe față de regulile tradiționale

Simplificări declarate: hartă pe pătrate (fără hexagoane), linia de vedere aproximată, un singur pachet de modificatori pentru echipă, fără monede/obiecte/niveluri.
