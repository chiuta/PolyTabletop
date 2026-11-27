# Corăbiile din Vale (`corabii_din_vale`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play corabii_din_vale --seed 1`. Sursa implementării: `polytabletop-games/src/c4_ceas.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cooperativ: trei bărci (10, 12 și 14 vâsle) trebuie să treacă valea înainte ca monstrul mării (amenințarea) să ajungă la 11. Arunci zarul și împingi o barcă (lucr t). Dacă ajung toate, câștigă cine a contribuit mai mult; altfel remiză.

Mutări: zar, lucr t.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
