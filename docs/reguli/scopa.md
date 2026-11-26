# Scopa (Italia) (`scopa`)

Joc italian de capturat cărți cu pachet de 40, pentru doi jucători, cu runde punctate și meci până la 11.

## Scop

Capturezi cât mai multe cărți valoroase și faci „scope” (golești masa); câștigi meciul ajungând la cel puțin 11 puncte.

## Pregătire

Pachet italian de 40 de cărți (`d c s b`, rangurile 1–7, F=8, C=9, R=10 care valorează 1–10 la adunare). Pe masă se pun 4 cărți și fiecare primește 3. Începe jucătorul care nu a împărțit; împărțitorul alternează la fiecare rundă.

## Desfășurarea unei ture

Joci o carte din mână. Dacă pe masă există o carte de aceeași valoare, trebuie să iei una dintre ele. Altfel, dacă există un grup de cărți a căror sumă este valoarea cărții tale, iei un asemenea grup. Dacă nu poți lua, lași cartea pe masă. Când ambele mâini se golesc, tastezi mutarea forțată `imparte` pentru încă 3 cărți fiecăruia, până se termină pachetul. La sfârșitul rundei se tastează `urmatoarea`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `7d` — lași cartea pe masă (doar dacă nu poți lua)
* `7d:3c+4b` — joci 7d și iei 3c și 4b de pe masă (suma 7)
* `7d:7s` — joci 7d și iei 7s de aceeași valoare
* `imparte` — mutare forțată: se dau încă 3 cărți
* `urmatoarea` — începe runda următoare după punctarea rundei

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play scopa --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `7b:7d` — joci 7 de bastoni și iei 7 de denari de pe masă (aceeași valoare).
2. `Cc` — joci Cavalul de coppe, lăsându-l pe masă (nu există captură).
3. `4d:4c` — joci 4 de denari și iei 4 de coppe; după aceasta mâinile se golesc și urmează `imparte`.

## Reguli speciale și variante alese

Captura unei cărți egale este obligatorie față de o sumă (dacă există carte egală nu poți lua un grup). Scopa nu se acordă la ultima carte a mâinii finale. Ultimul care a luat primește cărțile rămase pe masă (fără scopa). Nu se aplică regula de re-împărțire pentru trei sau mai mulți regi pe masă.

## Punctaj

La sfârșitul rundei: *carte* (cele mai multe cărți, peste 20) 1 punct; *denari* (cei mai mulți, peste 5) 1; *settebello* (7 de denari) 1; *primiera* 1 (suma celor mai bune cărți din fiecare culoare: 7=21, 6=18, 1=16, 5=15, 4=14, 3=13, 2=12, figurile=10; culoare lipsă = 0); fiecare scopa 1 punct.

## Sfârșitul jocului

Meciul se termină când cineva are cel puțin 11 puncte; la egalitate peste 11 se mai joacă o rundă.

## Note despre AI și informația ascunsă

Mâna adversarului nu apare în `render` (doar `J2 (ascuns) ține n cărți`). Împărțirea următoare este mutare forțată `imparte` separată, deci evaluarea nu vede viitorul. AI-ul folosește alpha-beta cu adâncime 1 și citește doar mâna proprie, masa și cărțile luate (publice).

## Diferențe față de regulile tradiționale

Varianta pentru doi jucători. Nu se aplică regula de re-împărțire pentru trei sau mai mulți regi. Împărțirea ca mutare explicită este o decizie de implementare.
