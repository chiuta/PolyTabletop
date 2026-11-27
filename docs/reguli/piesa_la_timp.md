# Piesa din Clepsidră (`piesa_la_timp`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play piesa_la_timp --seed 1`. Sursa implementării: `polytabletop-games/src/c4_capcana.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Ai o clepsidră cu limită ascunsă (5–12 ticuri). Fiecare act costă 1–3 ticuri și adaugă 1, 2 sau 4 piese în pot. Dacă depășești limita pierzi potul; cu stop îl bagi în scor. 8 ture; cel mai mare scor câștigă.

Mutări: act k, stop, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
