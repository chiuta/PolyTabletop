# Durak (Rusia) (`durak`)

Joc rusesc de eliminare a cărților, varianta „podkidnoi”, pentru doi jucători, cu pachet de 36 de cărți.

## Scop

Rămâi primul fără cărți când pachetul s-a golit; cine rămâne cu cărți în mână este „durak” (pierde).

## Pregătire

36 de cărți (6 7 8 9 10 J Q K A; culorile c, d, h, s). Fiecare primește 6 cărți; următoarea carte se întoarce și dă atuul, rămânând jos (se trage ultima). Începe atacator cine are cel mai mic atu (sau Jucătorul 1 dacă nimeni nu are).

## Desfășurarea unei ture

Atacatorul pune o carte. Apărătorul o bate cu o carte mai mare de aceeași culoare sau cu un atu (un atu se bate doar cu un atu mai mare), ori ia cărțile de pe masă (`iau`). Atacatorul poate adăuga cărți cu valori deja aflate pe masă (în total cel mult 6 și cel mult câte cărți avea apărătorul; în prima rundă cel mult 5). Când totul e bătut și atacatorul nu mai adaugă, spune `gata`: cărțile se retrag, apărătorul devine atacator. După ce apărătorul ia, atacatorul mai poate adăuga cărți. După fiecare rundă se trage până la 6 cărți (întâi atacatorul) prin mutarea forțată `umple`; atuul întors se trage ultimul.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `6h` — atac sau adăugare: cartea 6 de cupă
* `9h>6h` — bate cartea 6h de pe masă cu 9h
* `iau` — apărătorul ia toate cărțile de pe masă
* `gata` — atacatorul termină runda
* `umple` — mutare forțată: se trage până la 6 cărți

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play durak --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `9c` — atacul cu 9 de treflă.
2. `gata` — după ce a fost bătut, spui `gata` pentru a încheia runda.
3. `umple` — `umple` completează mâna la 6 cărți din pachet; apoi continuă runda nouă.

## Reguli speciale și variante alese

Variantă „podkidnoi”: atacatorul aruncă pe rând, fără pase. Un atu se poate bate doar cu un atu mai mare. În prima rundă se pot adăuga cel mult 5 cărți.

## Punctaj

Nu există scor numeric; contează doar cine rămâne cu cărți.

## Sfârșitul jocului

Cu pachetul gol, cine rămâne primul fără cărți câștigă; cine rămâne cu cărți este „durak”. Dacă rămân fără cărți ambii, remiză.

## Note despre AI și informația ascunsă

Informația ascunsă: `render` arată mâna Jucătorului 1, numărul cărților celuilalt și cărțile de pe masă. `evaluate` citește doar mâna celui evaluat și informația publică. AI-ul folosește alpha-beta adâncime 1. Tragerea (`umple`) este mutare forțată care se introduce de către jucătorul la mutare.

## Diferențe față de regulile tradiționale

Varianta „podkidnoi” pentru doi jucători, fără pase; antetul sursei nu declară alte simplificări.
