# Tiến lên (`tienlen`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tienlen --seed 1`. Sursa implementării: `polytabletop-games/src/tienlen.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 13 fiecare (restul ascuns). Rang 3…A, 2; culoare ♠ < ♣ < ♦ < ♥. Combinații: carte, pereche, treime, careu, scară (3+, fără 2), 3+ perechi consecutive; aceeași formă și mai mare, sau pas. Tăieri pe 2: careu sau 3 perechi consecutive pe un 2, careu sau 4 perechi consecutive pe o pereche de 2; 4 perechi consecutive taie și careul. Victorie instantanee: patru de 2, șase perechi, scară 3…A. Primul la 3 mâini. Mutări: 3s, 9h 9d, 5c 6d 7h, pas.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Tiến lên (Vietnam) — joc de eliminare a cărților cu „tăieri” (chặt), 2 jucători.

Reguli folosite (varianta de bază sudică, adaptată pentru doi):
* 52 de cărți, 13 fiecăruia; celelalte 26 rămân nefolosite și ascunse. Rangul: 3 < 4 < … < K
< A < 2; culoarea: ♠ < ♣ < ♦ < ♥ (cupa e cea mai mare).
* Combinații: carte singură, pereche, treime, careu (tứ quý), scară de cel puțin 3 ranguri
consecutive (fără 2) și „perechi consecutive” (đôi thông) de cel puțin 3 perechi (fără 2).
Se răspunde cu aceeași formă și același număr de cărți, cu cartea cea mai mare mai mare
(rang, apoi culoare).
* Tăieri (chặt) ale cărților de 2: pe un 2 singur — careu sau 3+ perechi consecutive; pe o
pereche de 2 — careu sau 4+ perechi consecutive; pe un careu și pe 3 perechi consecutive —
4+ perechi consecutive (3 perechi consecutive sunt tăiate și de un careu).
* Cine nu poate / nu vrea spune `pas`; în doi, după pas adversarul conduce din nou. Prima mână:
cine are cea mai mică carte conduce și o include; apoi câștigătorul mâinii.
* Victorie instantanee (tới trắng) la împărțire: patru de 2, șase perechi sau scară completă
3…A (12 ranguri).
* Meci: primul la 3 mâini câștigate. Omise: penalizarea „2 rămas”, 3–4 jucători,
pierderea dreptului de a juca după pas, tăierea cu mai multă miză.
* Mutări: cărți separate prin spațiu: `3s`, `9h 9d`, `5c 6d 7h` (scară; culorile cărților
interioare se aleg automat — cele mai slabe; tu alegi culoarea cărții cele mai mari).
* Informație ascunsă: vezi doar mâna ta; AI-ul citește doar mâna proprie și cărțile jucate
(Hard/Expert: eșantionare Monte-Carlo a cărților necunoscute).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
