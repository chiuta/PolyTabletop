# Grădina de Corali (puzzle de plăci) (`gradina_corali`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gradina_corali --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 4×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 3 piese la fel: 3 p/piesă; vecini la fel: 1 p/pereche; piese A pe margine: 1 p/piesă.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «reef».

## Reguli

Fiecare își umple propria tablă 4×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 3 piese la fel: 3 p/piesă; vecini la fel: 1 p/pereche; piese A pe margine: 1 p/piesă.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
