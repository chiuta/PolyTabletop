# Orașul în Straturi (puzzle de plăci) (`oras_straturi`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oras_straturi --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; rând sau coloană cu toate felurile: 3 p; mai multe piese decât adversarul la un fel: 2 p/fel.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «my city».

## Reguli

Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; rând sau coloană cu toate felurile: 3 p; mai multe piese decât adversarul la un fel: 2 p/fel.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
