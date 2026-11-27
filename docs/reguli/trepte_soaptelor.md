# Treptele Șoaptelor (`trepte_soaptelor`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play trepte_soaptelor --seed 1`. Sursa implementării: `polytabletop-games/src/c4_memo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> 7 perechi de obiecte și 2 fantome cu fața în jos. O pereche găsită înaintează pionul tău cu 1–3 trepte (după obiect); o fantomă te coboară 2. Primul pion la treapta 14 câștigă.

Mutări: intorc i, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
