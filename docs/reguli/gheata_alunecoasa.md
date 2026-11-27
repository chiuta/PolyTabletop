# Pisoiul pe Gheață (`gheata_alunecoasa`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gheata_alunecoasa --seed 1`. Sursa implementării: `polytabletop-games/src/c4_urmarire.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Pe rând, un pește fuge spre ieșire (10) și un pisoi îl urmărește pe un coridor de gheață. Alegi o alunecare 1–3; cu cât e mai lungă, cu atât poate ieși mai scurtă (rezultatul se află la mutarea forțată arata). Peștele ieșit ia 2 puncte, prins dă pisoiului 3. Rolurile alternează, 4 runde.

Mutări: desli k, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
