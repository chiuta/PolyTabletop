# Planetele Verzi (terraformare) (`planete_verzi`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play planete_verzi --seed 1`. Sursa implementării: `polytabletop-games/src/planete_verzi.rs`.

> Rezumat din `polytabletop list`: Doi coloniști terraformează 8 planete de 7 tipuri (inel de tipuri 0..6); fiecare are un tip-casă (0 și 4). «colonizez k» costă 1 + distanța pe inel dintre tipul planetei și tipul-casă, minus nivelul tău de terraformare (minim 1), minereu și 2 credite; prima planetă e liberă oriunde, următoarele trebuie să fie vecine cu una a ta. «cercetez» (3 cunoștințe) urcă nivelul (max 3); «transform»: 2 minereu → 1 credit; «colectez»: +2 minereu, +1 credit. După 14 mutări fiecare câștigă cine are mai multe puncte.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «gaia project».

## Reguli

Doi coloniști terraformează 8 planete de 7 tipuri (inel de tipuri 0..6); fiecare are un tip-casă (0 și 4). «colonizez k» costă 1 + distanța pe inel dintre tipul planetei și tipul-casă, minus nivelul tău de terraformare (minim 1), minereu și 2 credite; prima planetă e liberă oriunde, următoarele trebuie să fie vecine cu una a ta. «cercetez» (3 cunoștințe) urcă nivelul (max 3); «transform»: 2 minereu → 1 credit; «colectez»: +2 minereu, +1 credit. După 14 mutări fiecare câștigă cine are mai multe puncte.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
