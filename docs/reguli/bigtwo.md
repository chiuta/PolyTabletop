# Big Two (Pusoy dos) (`bigtwo`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bigtwo --seed 1`. Sursa implementării: `polytabletop-games/src/bigtwo.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 13 fiecare (restul ascuns). Rang 3…A, 2; culoare ♦ < ♣ < ♥ < ♠. Carte, pereche, treime sau mână de 5: scară < culoare < full < careu+1 < scară de culoare. Dai aceeași formă mai mare ori pas; cine are cea mai mică carte începe cu ea. Mâna se termină la golirea cărților: perdantul primește cărțile rămase ca penalizare (dublu de la 10 cărți); la 30 de puncte pierde meciul (cel mult 8 mâini). Mutări: 3d, 9h 9s, 3d 4c 5h 6s 7d, pas.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Big Two / Pusoy Dos (China, Filipine) — joc de eliminare a cărților, 2 jucători.

Reguli folosite (varianta internațională cea mai răspândită, adaptată pentru doi):
* 52 de cărți, 13 fiecăruia; celelalte 26 rămân nefolosite și ascunse (în patru jucători s-ar
împărți toate). Rangul: 3 < 4 < … < K < A < 2 (2 e cea mai mare); culoarea: ♦ < ♣ < ♥ < ♠.
* Combinații: carte singură, pereche, treime și mâini de cinci cărți: scară (5 ranguri
consecutive în ordinea 3…2, fără întoarcere), culoare (5 de aceeași culoare), full (treime
+ pereche), careu + 1 carte, scară de culoare. Ordinea mâinilor de 5: scară < culoare <
full < careu < scară de culoare. Scările și scările de culoare se compară după cea mai
mare carte (rang, apoi culoare), culorile după culoare apoi cărțile, full-ul după treime,
careul după careu.
* Cine conduce joacă orice combinație; celălalt trebuie să dea o combinație **de același
număr de cărți și mai mare** (aceeași formă: pereche peste pereche, mână de 5 peste mână de
5 mai mică), ori `pas`. Dacă adversarul dă pas, conduci din nou. Prima mână: cine are cea
mai mică carte (3♦ de obicei) conduce și trebuie să o includă.
* Mâna se termină când cineva rămâne fără cărți; perdantul primește ca penalizare numărul de
cărți rămase (dublu de la 10 cărți în sus). Meciul se termină când un jucător ajunge la 30 de
puncte de penalizare (pierde) sau după 8 mâini (câștigă cine are mai puține).
* Omise: scările A-2-3-4-5 / 2-3-4-5-6, ordinea alternativă a culorilor (Pusoy Dos clasic),
regula „2 de la capăt”, 3–4 jucători.
* Mutări: cărți separate prin spațiu, ex. `3d`, `9h 9s`, `3d 4c 5h 6s 7d`, `pas`.
* Informație ascunsă: vezi doar mâna ta; AI-ul citește doar mâna proprie, cărțile jucate și
numărul cărților adversarului (Hard/Expert: eșantionare Monte-Carlo cu completări ale
cărților necunoscute).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
