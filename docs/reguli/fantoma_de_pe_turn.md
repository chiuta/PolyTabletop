# Fantoma de pe Turn (`fantoma_de_pe_turn`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fantoma_de_pe_turn --seed 1`. Sursa implementării: `polytabletop-games/src/c4_memo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cooperativ: 6 perechi de obiecte cu fața în jos. Întorci două cărți (intorc i, arata). Fiecare greșeală urcă fantoma pe turn (7 trepte). Dacă ajunge sus pierdeți amândoi (remiză); dacă găsiți toate perechile, câștigă cine are mai multe.

Mutări: intorc i, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
