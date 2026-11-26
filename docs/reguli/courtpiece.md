# Court Piece (Rang, Mendikot) (`courtpiece`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play courtpiece --seed 1`. Sursa implementării: `polytabletop-games/src/courtpiece.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, câte 26. Cine nu e dealer vede primele 5 cărți și alege atuul (rang h), apoi se împarte restul și el conduce. Răspunzi la culoare; atuul bate tot. Mâna o câștigă cine ia mai multe zeci (mendi): 1 p.; toate 4 zecile (Mendikot) 2 p.; toate levatele (Court) 3 p.; la 2–2 decid levatele. Primul la 5 puncte. Mutări: rang h, 10h, As.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Court Piece / Rang / Mendikot (India) — joc de levate cu zecile („mendi”), pentru 2 jucători.

Reguli folosite (din regulile publice ale jocului în patru, adaptate pentru doi):
* 52 de cărți, rang A (cel mai mare) … 2, toate se împart: câte 26 fiecăruia. Se dau mai
întâi 5 cărți fiecăruia; cine nu e dealer (alternativ la fiecare mână) le vede și **alege
atuul** (`rang h`), apoi se împart restul de cărți și el conduce prima levată.
* Câștigătorul unei levate conduce următoarea. Răspunzi obligatoriu la culoarea condusă dacă
o ai; altfel poți juca orice, iar atuul bate tot.
* Se numără zecile (mendi, 4 în total) din levatele luate. Cine are mai multe zeci câștigă
mâna: 1 punct; dacă ia toate cele 4 zeci (Mendikot) 2 puncte; dacă ia toate cele 26 de
levate (Court) 3 puncte. La 2–2 zeci decide numărul de levate; la egalitate perfectă
(13–13) mâna se anulează și se împarte din nou. Primul la 5 puncte câștigă.
* Mutări: `rang h` (atu: c/d/h/s) și cărțile (`10h`, `As`).
* Informație ascunsă: vezi doar mâna ta (la alegerea atuului vezi doar primele 5 cărți).
Cărțile adversarului se deduc din cărțile jucate; AI-ul nu le citește (Hard/Expert:
eșantionare Monte-Carlo).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
