# Butoiașul Corsarului (`butoiasul_corsar`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play butoiasul_corsar --seed 1`. Sursa implementării: `polytabletop-games/src/c4_capcana.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> 10 deschizături; într-una (ascunsă tuturor) sare corsarul. Pe rând înfigi o sabie (taie i); mutarea forțată arata spune dacă a sărit. Cine l-a trezit dă adversarului un punct și runda se reia. Primul la 3 puncte câștigă.

Mutări: taie i, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
