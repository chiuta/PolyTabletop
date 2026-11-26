# Orașul Comun (puzzle de plăci) (`orasul_comun`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play orasul_comun --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 4×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: mai multe piese decât adversarul la un fel: 4 p/fel; grupuri de cel puțin 2 piese la fel: 1 p/piesă.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «between two cities».

## Reguli

Fiecare își umple propria tablă 4×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: mai multe piese decât adversarul la un fel: 4 p/fel; grupuri de cel puțin 2 piese la fel: 1 p/piesă.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
