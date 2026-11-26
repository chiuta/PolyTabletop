# Treci pachetul (Pass the Parcel) (`parcel`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play parcel --seed 1`. Sursa implementării: `polytabletop-games/src/parcel.rs`.

> Rezumat din `polytabletop list`: Joc de petrecere cu 8 straturi: pasezi pachetul (gratis) sau îl păstrezi (costă 1); apoi muzica se poate opri (15% plus 10% la fiecare acțiune) și cel care ține pachetul desface stratul și ia premiul (−3 până la 5). Câștigă cine are mai multe puncte după 8 straturi. Mutări: pasez, pastrez, muzica.

## Scop

După 8 straturi ai mai multe puncte decât adversarul.

## Pregătire

Pachet cu 8 straturi cu premii ascunse (5, 4, 3, 3, 2, 1, −2, −3, în ordine amestecată); se vede doar premiul stratului curent.

## Desfășurarea turei

Cel care ține pachetul alege `pasez` (îl dă celuilalt) sau `pastrez` (−1 punct); apoi `muzica` (mutare separată): se poate opri și cel care ține desface stratul și ia premiul.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `pasez`
* `pastrez`
* `muzica`

## Simplificări și note

AI: compară valoarea așteptată din premiul curent și șansa publică de oprire; nu citește straturile următoare.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
