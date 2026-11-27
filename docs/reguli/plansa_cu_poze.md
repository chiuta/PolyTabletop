# Cartonașul cu Poze (`plansa_cu_poze`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play plansa_cu_poze --seed 1`. Sursa implementării: `polytabletop-games/src/c4_bingo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Fiecare are un cartonaș deschis cu 6 poze din 10. La scot ies două piese; tu iei una (ia 1 sau ia 2), cealaltă merge la adversar. O piesă se acoperă doar dacă o ai pe cartonaș. Primul cartonaș complet câștigă.

Mutări: scot, ia 1, ia 2.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
