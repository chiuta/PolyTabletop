# Pădurea Spiridușilor (puzzle de plăci) (`padurea_spiridusilor`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play padurea_spiridusilor --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 4×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: vecini diferiți: 2 p/pereche; diagonale la fel: 1 p/pereche.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «kodama: the tree spirits».

## Reguli

Fiecare își umple propria tablă 4×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: vecini diferiți: 2 p/pereche; diagonale la fel: 1 p/pereche.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
