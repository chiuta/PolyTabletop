# Târgul Mării (comerț pe mare) (`targul_marii`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play targul_marii --seed 1`. Sursa implementării: `polytabletop-games/src/targul_marii.rs`.

> Rezumat din `polytabletop list`: Negustori în 8 porturi pe un inel; portul i produce marfa i mod 4 (preț 1,2,3,4). Fiecare are 4 cărți de acțiune: «fondez k» (casă într-un port liber, vecin cu una a ta; prima oriunde; costă 2 monede, prima e gratis), «produc» (fiecare casă dă o marfă, max 6), «vand» (vinzi toate mărfurile; prețul × casele tale de acel fel) și «recuperez» (cărțile jucate revin în mână, +1 monedă). Când un jucător are 6 case, runda finală se încheie: 3 puncte/casă, +1 pe fel de marfă, +1 la 3 monede.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «concordia».

## Reguli

Negustori în 8 porturi pe un inel; portul i produce marfa i mod 4 (preț 1,2,3,4). Fiecare are 4 cărți de acțiune: «fondez k» (casă într-un port liber, vecin cu una a ta; prima oriunde; costă 2 monede, prima e gratis), «produc» (fiecare casă dă o marfă, max 6), «vand» (vinzi toate mărfurile; prețul × casele tale de acel fel) și «recuperez» (cărțile jucate revin în mână, +1 monedă). Când un jucător are 6 case, runda finală se încheie: 3 puncte/casă, +1 pe fel de marfă, +1 la 3 monede.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
