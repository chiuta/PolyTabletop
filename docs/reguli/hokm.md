# Hokm (`hokm`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hokm --seed 1`. Sursa implementării: `polytabletop-games/src/hokm.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, câte 13 (restul ascuns). Hakem vede mai întâi 5 cărți și alege atuul (hokm h); conduce prima levată. Răspunzi la culoare; atuul bate tot. 7 din 13 levate = 1 p., 13–0 = 2 p., hakem cu zero levate = 3 p.; hakem rămâne dacă câștigă. Primul la 7 puncte. Mutări: hokm h (c/d/h/s), 10h, As.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Hokm (Iran) — joc de levate cu atu ales de „hakem”, variantă pentru 2 jucători.

Reguli folosite (din regulile publice ale jocului în patru, adaptate pentru doi):
* 52 de cărți, rang A (cel mai mare) … 2. Hakem (conducătorul) primește mai întâi 5 cărți,
le vede și **alege atuul** (hokm); apoi fiecare are 13 cărți. Celelalte 26 rămân nefolosite
și ascunse (în jocul în patru s-ar împărți toate). Primul hakem e tras la sorți.
* Hakem conduce prima levată; câștigătorul unei levate conduce următoarea. Răspunzi
obligatoriu la culoarea condusă dacă o ai; altfel poți juca orice, iar atuul bate tot.
* O mână se câștigă cu cel puțin 7 din cele 13 levate: 1 punct; 13–0 („kot”) = 2 puncte;
dacă hakem nu ia nicio levată (hakem-kot) adversarul primește 3. Dacă hakem câștigă, rămâne
hakem; altfel funcția trece la celălalt. Primul la 7 puncte câștigă meciul.
* Mutări: `hokm h` (atu: c/d/h/s) și cărțile (`10h`, `As`).
* Informație ascunsă: vezi doar mâna ta (la alegerea atuului hakem vede doar primele 5 cărți).
AI-ul alege atuul doar din cele 5 cărți și joacă după mâna proprie, levatele publice și
(Hard/Expert) eșantionare Monte-Carlo a cărților necunoscute.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
