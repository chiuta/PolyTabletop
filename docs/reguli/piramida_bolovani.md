# Piramida de Bolovani (`piramida_bolovani`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play piramida_bolovani --seed 1`. Sursa implementării: `polytabletop-games/src/c4_stapel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Încărcați pe rând o piramidă cu pietricele (2), bolovani (3) și stânci (4). Limita ascunsă e între 12 și 26. Zarul decide câte piese pui sau dai. Cine o prăbușește (verific) ia piesele căzute. Primul fără piese câștigă.

Mutări: zar, pun w, ofer w, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
