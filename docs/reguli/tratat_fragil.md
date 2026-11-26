# Tratatul Fragil (război pe hartă) (`tratat_fragil`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tratat_fragil --seed 1`. Sursa implementării: `polytabletop-games/src/brand_war.rs`.

> Rezumat din `polytabletop list`: Aceeași hartă de 12 teritorii, dar fiecare e un centru și unitățile sunt armate de forță 1. La început primești armate până egalezi numărul centrelor tale (max 3) și le pui într-un teritoriu-casă («pun n»). Până la 3 mutări «mut a b unu»: o armată trece într-un vecin; forța ei = 1 + sprijinul (câte teritorii vecine țintei, ale tale, au ≥2 armate); apărătorul are forța lui + sprijin. Atacatorul ocupă dacă are strict mai multă forță; la egalitate se întoarce. Câștigi cu 7 teritorii din 12; «gata» încheie tura.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «diplomacy».

## Reguli

Aceeași hartă de 12 teritorii, dar fiecare e un centru și unitățile sunt armate de forță 1. La început primești armate până egalezi numărul centrelor tale (max 3) și le pui într-un teritoriu-casă («pun n»). Până la 3 mutări «mut a b unu»: o armată trece într-un vecin; forța ei = 1 + sprijinul (câte teritorii vecine țintei, ale tale, au ≥2 armate); apărătorul are forța lui + sprijin. Atacatorul ocupă dacă are strict mai multă forță; la egalitate se întoarce. Câștigi cu 7 teritorii din 12; «gata» încheie tura.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
