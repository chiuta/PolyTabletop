# Poteca Turmelor (drum cu vite) (`poteca_turmelor`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play poteca_turmelor --seed 1`. Sursa implementării: `polytabletop-games/src/poteca_turmelor.rs`.

> Rezumat din `polytabletop list`: Doi crescători duc vite spre depozitul de la capătul potecii (12 trepte). O acțiune pe rând: «merg n» (1–3 trepte; dacă aterizezi pe treapta adversarului îi plătești 1 monedă), «iau r» (cumperi o vită din rasa r = 1..4 cu 2 monede, doar pe treptele-piață 3, 6, 9) sau «lucrez» (+2 monede). La depozit livrezi automat: 3 puncte pentru fiecare rasă diferită + 1 pentru fiecare vită, +3 monede, și te întorci la start. După 3 livrări câștigă cine are mai multe puncte.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «great western trail».

## Reguli

Doi crescători duc vite spre depozitul de la capătul potecii (12 trepte). O acțiune pe rând: «merg n» (1–3 trepte; dacă aterizezi pe treapta adversarului îi plătești 1 monedă), «iau r» (cumperi o vită din rasa r = 1..4 cu 2 monede, doar pe treptele-piață 3, 6, 9) sau «lucrez» (+2 monede). La depozit livrezi automat: 3 puncte pentru fiecare rasă diferită + 1 pentru fiecare vită, +3 monede, și te întorci la start. După 3 livrări câștigă cine are mai multe puncte.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
