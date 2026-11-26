# Sic Bo (`sicbo`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sicbo --seed 1`. Sursa implementării: `polytabletop-games/src/sicbo.rs`.

> Rezumat din `polytabletop list`: Trei zaruri; duel contra „casei” cu 100 de jetoane, 12 runde (ordinea de pariere alternează). Pariuri (plăți X:1): mic 1, mare 1 (pierd la triplu), cifra N 1/2/3, dublu N 10, triplu N 180, oricare-triplu 30, suma S (4 și 17: 60 … 9–12: 6), pereche a-b 5. Cel mult 6 pariuri pe rundă, mize 1/5/10/25. Mutări: mic:5, cifra:3:5, suma:9:5, pereche:1-2:5, fin, arunc, urmatoarea. Câștigă cine are mai multe jetoane.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Sic Bo (China / Macao) — duel de pariuri pe trei zaruri, 2 jucători.

Reguli folosite (tabla standard de pariuri, plăți „X la 1”):
* Trei zaruri. Pariuri: `mic` (suma 4–10, pierde la triplu) 1:1; `mare` (11–17, pierde la
triplu) 1:1; `cifra N` — pe câte zaruri apare N: 1:1, 2:1 sau 3:1; `dublu N` (cel puțin două
zaruri cu N) 10:1; `triplu N` (toate trei) 180:1; `oricare-triplu` 30:1; `suma S` (4 și 17:
60:1; 5, 16: 30:1; 6, 15: 17:1; 7, 14: 12:1; 8, 13: 8:1; 9–12: 6:1); `pereche a-b` (două
cifre diferite apar printre zaruri) 5:1.
* Varianta de duel (adaptare, nu regulă tradițională): fără bancă-jucător, ambii pariază
contra „casei” cu 100 de jetoane, 12 runde; câștigă cine are mai multe jetoane la final.
Ordinea de pariere alternează pe runde; al doilea vede pariurile primului. Cel mult 6
pariuri pe rundă și jucător, mize 1 / 5 / 10 / 25.
* Mutări: `mic:5`, `mare:5`, `cifra:3:5`, `dublu:3:5`, `triplu:3:5`, `oricare-triplu:5`,
`suma:9:5`, `pereche:1-2:5`, `fin` (termini pariurile), `arunc`, `urmatoarea`.
* Zarurile sunt deterministe din sămânță, dar se aruncă doar la mutarea forțată `arunc`;
AI-ul nu le vede dinainte. AI: probabilitate aproximativă de a termina în față, din
valoarea așteptată și varianța exactă a pariurilor puse.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
