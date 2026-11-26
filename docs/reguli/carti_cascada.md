# Cărți în Cascadă (cursă de cărți) (`carti_cascada`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play carti_cascada --seed 1`. Sursa implementării: `polytabletop-games/src/carti_cascada.rs`.

> Rezumat din `polytabletop list`: Fiecare are un pachet de 40 de cărți (4 culori × 1..10), trase la întâmplare, și 3 «posturi» deschise. Pe masă sunt cel mult 4 stive comune (una pe culoare) care cresc 1, 2, 3…10. Mutări: «joc i» (postul i pe o stivă: 1 deschide o stivă, altă carte trebuie să urmeze exact vârful stivei de aceeași culoare; în loc apare o carte nouă) sau «schimb i» (schimbi postul cu o carte nouă din pachet). Cine își golește pachetul și posturile câștigă; după 400 de mutări decid cărțile jucate minus cele rămase.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «dutch blitz».

## Reguli

Fiecare are un pachet de 40 de cărți (4 culori × 1..10), trase la întâmplare, și 3 «posturi» deschise. Pe masă sunt cel mult 4 stive comune (una pe culoare) care cresc 1, 2, 3…10. Mutări: «joc i» (postul i pe o stivă: 1 deschide o stivă, altă carte trebuie să urmeze exact vârful stivei de aceeași culoare; în loc apare o carte nouă) sau «schimb i» (schimbi postul cu o carte nouă din pachet). Cine își golește pachetul și posturile câștigă; după 400 de mutări decid cărțile jucate minus cele rămase.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
