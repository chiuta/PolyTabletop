# Turnul Animalelor (`turnul_fermei`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play turnul_fermei --seed 1`. Sursa implementării: `polytabletop-games/src/c4_stapel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Pui animale de 4 mărimi pe un turn comun. Zarul (zar) hotărăște: pui 1, pui 2, pui 1 și dai 1 adversarului sau doar dai 1 (ofer w). Un animal mare peste unul mic e mai riscant. Cine dărâmă (verific) ia piesele căzute. Câștigă primul fără piese.

Mutări: zar, pun w, ofer w, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
