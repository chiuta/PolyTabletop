# Copiii Apără Căsuța (`copii_contra_stafiilor`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play copii_contra_stafiilor --seed 1`. Sursa implementării: `polytabletop-games/src/c4_ceas.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cooperativ: trei monștri (puteri 7, 8, 9) vin spre căsuță. Arunci zarul (zar) și lovești un monstru (lucr t). Amenințarea crește la fiecare rundă și la fața 1. Dacă ajunge la 9 pierdeți amândoi (remiză); dacă îi înfrângeți, câștigă cine a lovit mai mult.

Mutări: zar, lucr t.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
