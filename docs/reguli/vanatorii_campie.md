# Vânătorii de Câmpie (puzzle de plăci) (`vanatorii_campie`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vanatorii_campie --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 5×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 3 piese la fel: 2 p/piesă; mai multe piese decât adversarul la un fel: 3 p/fel; piesele A: +1 p fiecare.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «carcassonne: hunters and gatherers».

## Reguli

Fiecare își umple propria tablă 5×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 3 piese la fel: 2 p/piesă; mai multe piese decât adversarul la un fel: 3 p/fel; piesele A: +1 p fiecare.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
