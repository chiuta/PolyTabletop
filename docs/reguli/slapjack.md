# Slapjack (`slapjack`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play slapjack --seed 1`. Sursa implementării: `polytabletop-games/src/slapjack.rs`.

> Rezumat din `polytabletop list`: Joc de reflexe cu cărți adaptat pe ture: întorci cartea de sus pe teanc; când apare un valet celălalt poate da cu palma (65% reușită, altfel teancul îl ia cel care a întors valetul). Palma pe altceva costă o carte. Mutări: joc, palma, rezolv.

## Scop

Rămâi cu toate cărțile (adversarul rămâne fără) sau ai mai multe după 600 de mutări.

## Pregătire

52 de cărți împărțite 26–26 cu fața în jos; teanc comun în mijloc.

## Desfășurarea turei

`joc` (întorci cartea de sus); la un valet, celălalt poate `palma` (șansă 65%, rezolvarea e mutarea separată `rezolv`) sau continuă cu `joc`. Palma pe altă carte te costă cartea de sus.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `joc`
* `palma`
* `rezolv`

## Simplificări și note

Reflexele sunt simulate cu șansa de 65%. AI: decide din valoarea așteptată; nu vede cărțile viitoare.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
