# Memoria Luminilor (`memoria_luminilor`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play memoria_luminilor --seed 1`. Sursa implementării: `polytabletop-games/src/c4_secventa.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cine adaugă pune o culoare (r v a g) la coada secvenței; celălalt o repetă din memorie (secvența se ascunde). Greșeala dă o penalizare (3 = pierzi) și secvența se reia. AI-ul își amintește cel mult 7 semnale.

Mutări: adaug c, rep c.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
