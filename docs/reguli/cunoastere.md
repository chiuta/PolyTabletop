# Cupa Cunoașterii (întrebări, 6 felii) (`cunoastere`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cunoastere --seed 1`. Sursa implementării: `polytabletop-games/src/cunoastere.rs`.

> Rezumat din `polytabletop list`: Banca proprie de 126 de întrebări în 6 categorii; alege o categorie (alege istorie), răspunde a–d (raspund b), apoi verific. Corect = felia și încă o întrebare; greșit = trece tura. Prima 6/6 câștigă. AI-ul nu are cunoștințe (ghicește); pentru meci echilibrat jucați în doi oameni.

## Scop

Primul care colectează câte o felie din fiecare dintre cele 6 categorii câștigă.

## Pregătire

Bancă proprie de 126 de întrebări în 6 categorii: geografie, istorie, stiinta, natura, arta, general. Fără tablă și fără zar.

## Desfășurarea turei

Alegi o categorie pentru care încă nu ai felia; apare o întrebare cu variantele a–d (ordinea e amestecată la fiecare întrebare); răspunzi, apoi mutarea forțată `verific` arată rezultatul. Corect: primești felia și continui; greșit: tura trece.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `alege istorie`.
* `raspund b`.
* `verific` (forțată).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (6): `alege geografie`, `alege istorie`, `alege stiinta`, `alege natura`, `alege arta`, `alege general`. Introduci `alege geografie`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (4): `raspund a`, `raspund b`, `raspund c`, `raspund d`. Introduci `raspund c`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `verific`. Introduci `verific`; răspunsul calculatorului (în ordine): `alege general`; `raspund c`; `verific`.

## Reguli speciale și variante alese

* Întrebările nu se repetă până se epuizează categoria, apoi se amestecă din nou.

## Punctaj

Feliile colectate (6 la câștig).

## Sfârșitul jocului

Când un jucător are cele 6 felii.

## Note despre AI

Varianta corectă stă în starea jocului, dar `evaluate` nu o citește: AI-ul nu are cunoștințe, ghicește (aproximativ 25% șanse) și își alege categoria la întâmplare. Pentru un meci echilibrat jucați în doi oameni. Modul `--you both` arată întrebarea oricui e la rând, dar nu și răspunsul corect.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Alegerea categoriei ține loc de mers pe tablă.
