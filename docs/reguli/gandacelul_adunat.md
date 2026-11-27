# Gândăcelul din Piese (`gandacelul_adunat`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gandacelul_adunat --seed 1`. Sursa implementării: `polytabletop-games/src/c4_bingo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Fiecare își adună un gândăcel: corp, cap, 2 ochi, 2 antene, gură, 2 picioare; capul și restul cer corpul întâi. La scot ies două piese: iei una (ia 1/ia 2), cealaltă merge la adversar. Primul gândăcel complet câștigă.

Mutări: scot, ia 1, ia 2.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
