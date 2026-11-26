# Atlasul Pierdut (puzzle de plăci) (`atlas_pierdut`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play atlas_pierdut --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 5×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană dintr-un singur fel: 6 p; seturi complete: 3 p/set; vecini diferiți: 1 p/pereche.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «cartographers».

## Reguli

Fiecare își umple propria tablă 5×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană dintr-un singur fel: 6 p; seturi complete: 3 p/set; vecini diferiți: 1 p/pereche.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
