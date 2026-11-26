# Tablanet (`tablanet`)

Tablanet (Table), joc românesc de capturat cărți, pentru doi jucători, cu 52 de cărți.

## Scop

Aduni cele mai multe puncte din cărți capturate, table și cărți speciale.

## Pregătire

Pachet de 52. Se pun 4 cărți pe masă, apoi câte 6 fiecăruia; când mâinile se golesc se mai dau câte 6, până se termină pachetul (4 runde de 6). `imparte` este mutarea forțată de distribuire.

## Desfășurarea unei ture

Joci o carte. Dacă poate captura (carte de aceeași valoare sau grupuri care se adună la valoarea ei), ești obligat să capturezi, și anume numărul maxim posibil de cărți (între capturi egale alegi tu). Dacă nu poți captura, lași cartea pe masă.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `7h` — lași cartea pe masă (doar dacă nu poți captura)
* `7h x 3c 4d` — joci 7h și capturezi 3c și 4d
* `imparte` — împarte următoarele 6 cărți

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tablanet --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `8c x 2d 6d` — joci 8 de treflă și capturezi 2d și 6d (2+6=8, captura maximă).
2. `9h` — joci 9 de cupă pe masă (fără captură posibilă).
3. `Ks` — joci Regele de pică pe masă.

## Reguli speciale și variante alese

Valori: A = 1 sau 11, 2–10 numeric. J, Q, K nu se adună: capturează doar cărți de același rang. Masa curățată = „tablă”, dar nu la ultima captură a jocului. Ultimul care a capturat ia cărțile rămase pe masă.

## Punctaj

Cele mai multe cărți 3 (egalitate: nimic), cele mai multe treflă 1, 10♦ = 2, 2♣ = 1, fiecare As 1, fiecare tablă 1.

## Sfârșitul jocului

La sfârșitul pachetului câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

AI-ul (alpha-beta adâncime 1) citește mâna proprie, masa și cărțile capturate (publice). Mâna adversarului apare doar ca număr de cărți.

## Diferențe față de regulile tradiționale

Joc pentru doi; sursa nu declară alte simplificări.
