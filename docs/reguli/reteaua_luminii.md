# Rețeaua Luminii (economie energetică) (`reteaua_luminii`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play reteaua_luminii --seed 1`. Sursa implementării: `polytabletop-games/src/reteaua_luminii.rs`.

> Rezumat din `polytabletop list`: Duel economic: 8 orașe pe un inel, 4 centrale la vânzare, combustibil la preț variabil. Faci o acțiune pe rând: «centrala i» (cumperi o centrală, ai cel mult 2), «combustibil c|p» (cărbune/petrol; energia verde nu are nevoie), «oras k» (conectezi orașul: 5 + costul legăturii cu rețeaua ta, +4 dacă îl are adversarul), «produc» (arzi combustibil, alimentezi orașe și încasezi 4 + 3 pe oraș) sau «pas». Primul cu 7 orașe declanșează sfârșitul; câștigă cine alimentează mai multe orașe, apoi cine are mai mulți bani.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «power grid».

## Reguli

Duel economic: 8 orașe pe un inel, 4 centrale la vânzare, combustibil la preț variabil. Faci o acțiune pe rând: «centrala i» (cumperi o centrală, ai cel mult 2), «combustibil c|p» (cărbune/petrol; energia verde nu are nevoie), «oras k» (conectezi orașul: 5 + costul legăturii cu rețeaua ta, +4 dacă îl are adversarul), «produc» (arzi combustibil, alimentezi orașe și încasezi 4 + 3 pe oraș) sau «pas». Primul cu 7 orașe declanșează sfârșitul; câștigă cine alimentează mai multe orașe, apoi cine are mai mulți bani.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
