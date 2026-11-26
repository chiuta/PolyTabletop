# Barbut (duel simplificat) (`barbut`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play barbut --seed 1`. Sursa implementării: `polytabletop-games/src/barbut.rs`.

> Rezumat din `polytabletop list`: 10 jetoane fiecare. Pariu 1–3; prima aruncare: 7/11 câștigă, 2/3/12 pierde, altfel punct; adversarul plătește (dublează) sau lasă; apoi punctul bate 7. Mutări: pariez 1, arunc, plătesc, las.

## Scop

Ai la final mai multe jetoane decât adversarul (sau îl lași fără jetoane).

## Pregătire

Fiecare jucător pornește cu 10 jetoane. Aruncătorul alternează la fiecare rundă.

## Desfășurarea turei

Fazele unei runde: (1) Pariu: aruncătorul alege miza `pariez 1`–`3`, ambii pun miza în pot. (2) Zar de început: `arunc` cu două zaruri: 7 sau 11 = aruncătorul câștigă potul; 2, 3 sau 12 = îl pierde; altă sumă devine „punct”. (3) Decizie (după un punct): adversarul alege `plătesc` (dublează miza, cât permit jetoanele) sau `las` (pierde miza). (4) Aruncări: `arunc` până cade din nou punctul (aruncătorul câștigă) sau 7 (pierde).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `pariez 1`, `pariez 2`, `pariez 3`
* `arunc`
* `plătesc` (cu diacritică) și `las`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `pariez 1`, `pariez 2`, `pariez 3`. Introduci `pariez 1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): `plătesc`.
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Miza este limitată de jetoanele ambilor jucători.

## Punctaj

Jetoanele câștigate sau pierdute în pot.

## Sfârșitul jocului

Când un jucător rămâne fără jetoane sau după 40 de runde (câștigă cine are mai multe jetoane; egalitate = remiză).

## Note despre AI

Zarurile sunt aruncate automat de joc. În fazele Pariu și Decizie zarurile următoare NU sunt afișate; în fazele cu mutarea unică `arunc` se afișează. AI-ul (adâncime 1) evaluează după speranța matematică (probabilitatea de a atinge punctul înaintea lui 7) și nu poate vedea rezultatul zarurilor.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Duel simplificat; barbutul real se joacă cu mai mulți pariori și cu „banca”.
* Pasul `plătesc`/`las` (dublarea mizei) și limita de 3 jetoane sunt adaosuri ale acestei variante, nu reguli atestate.
