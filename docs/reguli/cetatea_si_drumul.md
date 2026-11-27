# Drumul spre Cetate (`cetatea_si_drumul`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cetatea_si_drumul --seed 1`. Sursa implementării: `polytabletop-games/src/c4_mozaic.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Tablă comună 5×5. Din piața de 3 plăci iei una și o așezi lipită de cele existente (j r c). Primești 1 plus vecinii de același fel; cu f pui și un însoțitor (3 pe jucător) care la final ia mărimea grupului lui. 10 plăci pe jucător.

Mutări: j r c [f], pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
