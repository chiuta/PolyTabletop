# Capra (Leapfrog) (`capra`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play capra --seed 1`. Sursa implementării: `polytabletop-games/src/capra.rs`.

> Rezumat din `polytabletop list`: Cursă pe un șir de 10 căsuțe: 3 săritori fiecare, care merg spre partea opusă. Faci un pas pe căsuța vecină liberă sau sari peste un săritor alăturat pe căsuța liberă din spatele lui; câștigă primul cu toți cei trei pe ultimele căsuțe. Mutări: 3-4, 2-4, pas.

## Scop

Primul care aduce toți cei 3 săritori pe ultimele 3 căsuțe din partea opusă.

## Pregătire

Șir de 10 căsuțe; J1 pe 1–3 (spre 10), J2 pe 8–10 (spre 1).

## Desfășurarea turei

Un pas înainte pe căsuță liberă (`3-4`) sau săritură peste un săritor vecin (`2-4`); fără mers înapoi; `pas` când nu ai mutare.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `3-4`
* `2-4`

## Simplificări și note

Dacă nimeni nu mai poate muta, câștigă cine a înaintat mai mult. AI: alfa-beta.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
