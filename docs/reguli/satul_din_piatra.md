# Satul din Piatră și Lemn (`satul_din_piatra`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play satul_din_piatra --seed 1`. Sursa implementării: `polytabletop-games/src/c4_ceas.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cooperativ: construiți 4 clădiri (coliba, focul, câmpul, podul) înainte ca amenințarea să ajungă la 11. Cu zarul culegi resurse (culeg r) sau cheltuiești (constr t). Dacă ați terminat, câștigă cine a contribuit mai mult; altfel remiză.

Mutări: zar, culeg r, constr t.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
