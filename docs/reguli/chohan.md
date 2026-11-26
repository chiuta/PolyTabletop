# Chō-han (Chō-han bakuchi) (`chohan`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chohan --seed 1`. Sursa implementării: `polytabletop-games/src/chohan.rs`.

> Rezumat din `polytabletop list`: Pariu japonez cu zaruri: două zaruri sub un pahar, pariezi cho (pară) sau han (impară) cu 5, 10 sau 20 de jetoane; celălalt primește partea opusă. 10 runde, 100 de jetoane fiecare, chemătorul alternează. Mutări: cho 10, han 5, agit.

## Scop

Ai mai multe jetoane după 10 runde.

## Pregătire

100 de jetoane fiecare; „chemătorul” alternează în fiecare rundă.

## Desfășurarea turei

Chemătorul spune `cho 10` sau `han 5` (mize 5, 10, 20); celălalt primește partea opusă cu aceeași miză; `agit` (mutare separată) ridică paharul. Cine a nimerit partea ia miza.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `cho 10`
* `agit`

## Simplificări și note

Zarurile sunt aruncate la `agit`, deci AI-ul nu le vede în avans; cho și han au aceeași probabilitate, deci AI-ul alege la întâmplare.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
