# Magistrale (rute cu cărți de vagon) (`magistrale`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play magistrale --seed 1`. Sursa implementării: `polytabletop-games/src/magistrale.rs`.

> Rezumat din `polytabletop list`: Joc original: hartă cu 19 orașe și 42 de rute; strângi cărți de vagon (r a v g m, j = locomotivă) și ocupi rute (ocupa 12 r), tragi 2 cărți (trage / trage r) sau bilete de destinație secrete (bilete, pastrez 1,3). La final bilet îndeplinit +valoarea, neîndeplinit −valoarea, drum lung +10. În doi, rutele duble se ocupă o singură dată. Informație ascunsă: vezi doar cărțile și biletele tale.

## Scop

Ai cele mai multe puncte la final: rute ocupate, bilete îndeplinite și cel mai lung drum.

## Pregătire

Hartă cu 19 orașe și 42 de rute (38 de rute plus 4 paralele din rutele duble). Cărți de vagon în 5 culori (r roșu, a albastru, v verde, g galben, m mov), câte 12, plus 10 locomotive `j` (joker); 5 cărți vizibile (dacă trei sunt locomotive, se înlocuiesc toate). 40 de vagoane fiecare; începi cu 4 cărți și 3 bilete de destinație din care păstrezi cel puțin 2. Biletele sunt secrete.

## Desfășurarea turei

Alegi exact una: tragi 2 cărți (`trage` din pachet sau `trage r` vizibilă; o locomotivă vizibilă se ia singură), ocupi o rută sau tragi bilete noi.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trage`, `trage r`, `trage j`.
* `ocupa 12 r` (ruta 12 cu cărți din culoarea r; rutele gri merg cu orice culoare; locomotivele țin loc de orice).
* `bilete` (tragi 3, păstrezi cel puțin unul); alegerea: `pastrez 1,3`.
* `pas` (dacă nu ai altă mutare).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (4): `pastrez 1,2`, `pastrez 1,3`, `pastrez 2,3`, `pastrez 1,2,3`. Introduci `pastrez 1,2`; răspunsul calculatorului (în ordine): `pastrez 2,3`.
2. Mutări legale (19): `trage`, `trage g`, `trage j`, `trage v`, `trage r`, `ocupa 0 a`, `ocupa 0 v`, `ocupa 0 g`, `ocupa 9 v`, `ocupa 13 v` …. Introduci `ocupa 13 v`; răspunsul calculatorului (în ordine): `ocupa 41 m`.
3. Mutări legale (12): `trage`, `trage g`, `trage j`, `trage v`, `trage r`, `ocupa 0 a`, `ocupa 0 g`, `ocupa 31 a`, `ocupa 31 g`, `ocupa 32 a` …. Introduci `bilete`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Scorul rutei: 1, 2, 4, 7, 10, 15 puncte pentru lungimea 1–6.
* În doi, la rutele duble doar una dintre ele poate fi ocupată.
* Valoarea unui bilet = lungimea celui mai scurt drum dintre orașe.

## Punctaj

Rute ocupate + bilete îndeplinite (+valoarea) − bilete neîndeplinite (−valoarea) + cel mai lung drum continuu +10 (egalitate: ambii).

## Sfârșitul jocului

Când cuiva îi rămân cel mult 2 vagoane, adversarul mai joacă o ultimă mutare. Limită de siguranță: 1200 de mutări.

## Note despre AI

Informație ascunsă: vezi doar cărțile și biletele tale. AI-ul nu se uită la mâna adversarului sau la ordinea pachetului; la evaluare ignoră cărțile abia trase „pe orb”.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Joc original (hartă, bilete, texte proprii) cu mecanică generică de colectare de cărți și ocupare de rute.
