# Insula Puilor de Dragon (`insula_dragonilor`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play insula_dragonilor --seed 1`. Sursa implementării: `polytabletop-games/src/c4_mozaic.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Fiecare își face propria insulă 5×5 din domino cu două terenuri și ouă de dragon. Din piața de 3 iei una și o așezi (j r c o); trebuie să atingă un teren la fel. Câștigă cine are mai multe ouă după 9 plăci.

Mutări: j r c o, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
