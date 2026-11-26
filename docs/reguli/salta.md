# Salta (`salta`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play salta --seed 1`. Sursa implementării: `polytabletop-games/src/salta.rs`.

> Rezumat din `polytabletop list`: Jocul lui Büttgenbach (1899) pe 10×10: fără capturi; duci toate cele 15 piese pe pătratele de start ale adversarului. Pași diagonali (în orice direcție) sau salturi peste o piesă adversă înainte; săritura e obligatorie. Mutări: a1-b2, c3xe5.

## Scop

Primul care ocupă toate cele 15 pătrate de start ale adversarului.

## Pregătire

10×10, doar pătratele întunecate; câte 15 piese pe primele trei rânduri ale fiecăruia.

## Desfășurarea turei

Pas pe diagonală (înainte sau înapoi) pe un pătrat gol, sau săritură înainte peste o piesă adversă (piesa sărită rămâne); săritura e obligatorie când există.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `a1-b2`
* `c3xe5`

## Simplificări și note

Simplificare: o singură săritură pe mutare, fără regula „ordinii pieselor”. După 240 de semi-mutări câștigă cine mai are de parcurs mai puțin.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
