# Curtea Reginei (puzzle de plăci) (`curtea_reginei`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play curtea_reginei --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 5×5 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; piesele A: +2 p fiecare; patru colțuri la fel: 5 p.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «queendomino».

## Reguli

Fiecare își umple propria tablă 5×5 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; piesele A: +2 p fiecare; patru colțuri la fel: 5 p.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
