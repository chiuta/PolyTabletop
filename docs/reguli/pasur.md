# Pasur (`pasur`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pasur --seed 1`. Sursa implementării: `polytabletop-games/src/pasur.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 4 pe masă și 4 în mână (se dau din nou câte 4). Un număr (A=1…10) capturează orice grupuri de cărți de pe masă cu suma 11 împreună cu el; Valetul ia tot în afară de Dame și Regi; Dama ia o Damă, Regele un Rege. Sur (masa golită, nu cu Valet) 5 p. Puncte: cele mai multe cărți 3, ≥7 trefle 7, 10♦ 3, 2♣ 2, Ași 1, Valeți 1. Joc până la 62. Mutări: 7h (pui), 5s x 2c 4d 6h (capturezi), imparte.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Pasur (Iran) — joc de capturi cu sume de 11, 2 jucători.

Reguli folosite (varianta de bază din regulile publice):
* 52 de cărți. Pe masă se pun 4 cărți cu fața în sus (fără valeți; dacă apare un valet se
reamestecă); fiecare primește 4 cărți, iar când mâinile se golesc se dau încă 4 (6 runde
de câte 8 cărți). Cine nu e dealer începe; dealerul alternează între împărțiri.
* La mutare joci o carte. **Cărți de la A la 10** (A = 1): capturează orice grupuri de cărți
de pe masă (A–10) a căror sumă împreună cu cartea ta e 11 (poți lua mai multe grupuri
deodată, fiecare cu suma 11 − cartea ta). **Valetul (J)** capturează toate cărțile de pe
masă în afară de Dame și Regi. **Dama** capturează o Damă, **Regele** un Rege de pe masă.
Cartea jucată merge și ea în grămada ta. Dacă nu poți / nu vrei să capturezi, cartea
rămâne pe masă (în această variantă capturarea nu e obligatorie).
* „Sur”: dacă o captură (nu cu valet) golește masa, primești 5 puncte la sfârșit (nu și la
ultima carte a împărțirii). La sfârșitul împărțirii cărțile rămase pe masă merg la ultimul
care a capturat.
* Puncte pe împărțire: cele mai multe cărți 3 (la egalitate nimeni); cel puțin 7 trefle 7;
10♦ 3; 2♣ 2; fiecare As 1; fiecare Valet 1; fiecare sur 5. Se joacă împărțiri până când
cineva ajunge la 62 de puncte (mai mare câștigă; la egalitate se mai joacă o împărțire).
* Mutări: `7h` (doar o pui pe masă) sau `7h x 4c` / `5s x 2c 4d 6h` (capturezi cărțile
listate); `imparte` după ce se golesc mâinile.
* Informație ascunsă: vezi doar mâna ta; masa și grămezile capturate sunt publice. AI-ul
folosește doar mâna proprie, masa, grămezile și (Hard/Expert) eșantionare Monte-Carlo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
