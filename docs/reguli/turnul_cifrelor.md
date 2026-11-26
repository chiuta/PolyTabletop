# Turnul Cifrelor (puzzle de plăci) (`turnul_cifrelor`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play turnul_cifrelor --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 4×5 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: diagonale la fel: 2 p/pereche; rând sau coloană dintr-un singur fel: 6 p; vecini diferiți: 1 p/pereche.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «nmbr 9».

## Reguli

Fiecare își umple propria tablă 4×5 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: diagonale la fel: 2 p/pereche; rând sau coloană dintr-un singur fel: 6 p; vecini diferiți: 1 p/pereche.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
