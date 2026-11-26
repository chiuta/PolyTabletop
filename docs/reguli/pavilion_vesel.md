# Pavilionul Vesel (puzzle de plăci) (`pavilion_vesel`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pavilion_vesel --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 5×4 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 5 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană cu toate felurile: 3 p; vecini la fel: 1 p/pereche; mai multe piese decât adversarul la un fel: 2 p/fel.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «azul: summer pavilion».

## Reguli

Fiecare își umple propria tablă 5×4 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 5 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană cu toate felurile: 3 p; vecini la fel: 1 p/pereche; mai multe piese decât adversarul la un fel: 2 p/fel.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
