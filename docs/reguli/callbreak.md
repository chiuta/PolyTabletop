# Call Break (`callbreak`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play callbreak --seed 1`. Sursa implementării: `polytabletop-games/src/callbreak.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, câte 13 (restul ascuns); pica e mereu atu. Cine nu e dealer licitează primul (licit 4) numărul de levate și conduce. Răspunzi la culoare și, dacă poți, cu o carte mai mare; fără culoare tai cu pică. Scor: licitația + 0,1 pe levată în plus, sau − licitația dacă ai luat mai puțin. 5 mâini, scorul total mai mare câștigă. Mutări: licit 4, 10h, As.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Call Break (Nepal / India) — joc de levate cu licitație exactă și pică ca atu, 2 jucători.

Reguli folosite (din regulile publice ale jocului în patru, adaptate pentru doi):
* 52 de cărți, rang A (cel mai mare) … 2; **pica (♠) este mereu atu**. Fiecare primește 13
cărți; celelalte 26 rămân nefolosite și ascunse (în patru jucători s-ar împărți toate).
* Licitația: fiecare spune (`licit N`, 1–13) câte levate se angajează să ia; cel care nu e
dealer licitează primul și conduce prima levată; câștigătorul unei levate conduce următoarea.
* Joc: răspunzi la culoarea condusă și, dacă poți, **trebuie să dai o carte mai mare**;
dacă n-ai culoarea, **trebuie să tai cu pică** dacă ai; altfel dai orice.
* Scor pe mână (în zecimi, afișat ca la hârtie): cine ia cel puțin cât a licitat primește
licitația + 0,1 pentru fiecare levată în plus (ex.: 4 licitate, 6 luate = 4,2); cine ia mai
puțin pierde licitația (−4). Meciul are 5 mâini; câștigă cine are scorul total mai mare
(egalitate = remiză).
* Mutări: `licit 4`, apoi cărțile (`10h`, `As`).
* Informație ascunsă: vezi doar mâna ta și licitația adversarului. AI-ul își estimează
numărul de levate din propria mână și joacă după levate rămase; la Hard/Expert folosește
eșantionare Monte-Carlo a cărților necunoscute.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
