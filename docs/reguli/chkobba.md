# Chkobba (Tunisia) (`chkobba`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chkobba --seed 1`. Sursa implementării: `polytabletop-games/src/scopa.rs (Rules::Chkobba)`.

> Rezumat din `polytabletop list`: Pachet de 40 de cărți (1–7, damă, valet, rege), 3 în mână, 4 pe masă. Iei o carte de aceeași valoare, altfel un grup cu aceeași sumă; golești masa = chkobba (nu la ultima carte). Punctaj: carte, denari, bermila (cei mai mulți de 7, apoi de 6), haya (7 de denari), chkobba; meci până la 11. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: 7d, 7d:3c+4b, imparte, urmatoarea.

## Scop

Aduni puncte pe runde până la 11.

## Pregătire

Pachet de 40 de cărți: culori `d` (denari/monede), `c` (cupe), `s` (săbii), `b` (bastoane); ranguri 1–7, 8 = damă (F), 9 = valet (C), 10 = rege (R). 4 cărți pe masă, 3 în mână; a doua împărțire etc. cu mutarea forțată `imparte`.

## Desfășurarea turei

Joci o carte. Dacă pe masă e o carte de aceeași valoare, trebuie să o iei; altfel poți (trebuie) să iei un grup de cărți cu aceeași sumă; altfel o lași pe masă. Golirea mesei = chkobba (1 punct, nu la ultima carte a rundei); ultimul care a capturat ia ce a rămas pe masă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `7d`: lași cartea pe masă; `7d:3c+4b`: iei 3c și 4b; `7d:7s`: iei 7s.
* `imparte` și `urmatoarea` (mutări forțate).

## Reguli speciale și variante alese

* Reutilizează motorul `scopa` (`Rules::Chkobba`): aceleași capturi, alt punctaj.

## Punctaj

Carte (cele mai multe cărți), denari (cei mai mulți), bermila (cele mai multe de 7; la egalitate 2–2, cele mai multe de 6), haya (7 de denari), 1 punct pe chkobba.

## Sfârșitul jocului

Meciul se joacă până la 11 puncte (la egalitate peste 11 se mai joacă o rundă).

## Note despre AI

Informație ascunsă (mâna adversarului și pachetul); evaluarea citește doar informația publică. Alpha-beta cu adâncime 1.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; mâna afișată este a jucătorului la mutare, cu un banner de predare a ecranului.

## Diferențe față de regulile tradiționale

* Variante locale: regula de re-împărțire pentru regii de pe masă nu se aplică; valorile cărților figurate folosesc eticheta F/C/R a motorului Scopa.
