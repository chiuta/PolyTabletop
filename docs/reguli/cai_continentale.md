# Căi Ferate Continentale (rute feroviare) (`cai_continentale`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cai_continentale --seed 1`. Sursa implementării: `polytabletop-games/src/brand_rail.rs`.

> Rezumat din `polytabletop list`: Hartă mare de 14 orașe și 24 de rute. Strângi cărți colorate (r, v, a, g, n; J = joker) și ocupi rute plătind atâtea cărți cât e lungimea, de culoarea rutei (rutele «*» acceptă orice culoare). Fiecare are 3 bilete publice: conectate = bonus, neconectate = pierdere. Mutări: «ocup n», «trag i» (cartea vizibilă i plus una oarbă), «trag oarba». Ai 24 de trenuri; sfârșit când cineva rămâne cu 2 sau mai puține. Scor pe rute după lungime.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «ticket to ride: europe».

## Reguli

Hartă mare de 14 orașe și 24 de rute. Strângi cărți colorate (r, v, a, g, n; J = joker) și ocupi rute plătind atâtea cărți cât e lungimea, de culoarea rutei (rutele «*» acceptă orice culoare). Fiecare are 3 bilete publice: conectate = bonus, neconectate = pierdere. Mutări: «ocup n», «trag i» (cartea vizibilă i plus una oarbă), «trag oarba». Ai 24 de trenuri; sfârșit când cineva rămâne cu 2 sau mai puține. Scor pe rute după lungime.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
