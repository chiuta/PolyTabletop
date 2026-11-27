# Parcul cu Perechi (`parcul_perechilor`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play parcul_perechilor --seed 1`. Sursa implementării: `polytabletop-games/src/c4_mozaic.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Fiecare are un parc propriu de 4×3. Din piața de 3 plăci cu perechi de animale iei una și o așezi (j r c o). La final primești 2 puncte pentru fiecare vecinătate de același animal. 6 plăci pe jucător.

Mutări: j r c o, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
