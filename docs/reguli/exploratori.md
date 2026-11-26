# Exploratorii Templului (puzzle de plăci) (`exploratori`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play exploratori --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: patru colțuri la fel: 8 p; vecini la fel: 1 p/pereche; mai multe piese decât adversarul la un fel: 3 p/fel.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «karuba».

## Reguli

Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: patru colțuri la fel: 8 p; vecini la fel: 1 p/pereche; mai multe piese decât adversarul la un fel: 3 p/fel.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
