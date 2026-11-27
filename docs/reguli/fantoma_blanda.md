# Fantoma cea Blândă (`fantoma_blanda`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fantoma_blanda --seed 1`. Sursa implementării: `polytabletop-games/src/c4_pista.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Fantoma (alternativ) știe o țintă dintre 12 (vis, culoare) și dă cel mult 2–3 indicii adevărate (ind A=v, gata); celălalt ghicește (ghic k, verific). Ținta e vizibilă doar fantomei. 6 runde; rolurile alternează.

Mutări: ind A=v, gata, ghic k, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
