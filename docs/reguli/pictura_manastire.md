# Pictura Mănăstirii (puzzle de plăci) (`pictura_manastire`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pictura_manastire --seed 1`. Sursa implementării: `polytabletop-games/src/brand_grid.rs`.

> Rezumat din `polytabletop list`: Fiecare își umple propria tablă 5×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: seturi complete: 5 p/set; rând sau coloană dintr-un singur fel: 4 p; piesele A: +1 p fiecare.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «fresco».

## Reguli

Fiecare își umple propria tablă 5×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: seturi complete: 5 p/set; rând sau coloană dintr-un singur fel: 4 p; piesele A: +1 p fiecare.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
