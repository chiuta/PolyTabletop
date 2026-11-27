# Trezirea Reginelor (`trezirea_reginelor`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play trezirea_reginelor --seed 1`. Sursa implementării: `polytabletop-games/src/c4_regine.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> 12 regine dorm cu fața în jos. Cu un rege trezești o regină (valoarea apare abia la mutarea forțată arata), cu un cavaler furi o regină trează (un dragon o apără), cu o poțiune o adormi (o baghetă o apără). Aruncă numere singure, în pereche egală sau ca sumă a b c. Mâna ta e secretă. Câștigi cu 50 de puncte sau 5 regine.

Mutări: rei s, cav q, pot q, arunc c [c], suma a b c, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
