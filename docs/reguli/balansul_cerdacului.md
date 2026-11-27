# Balanța Cerdacului (`balansul_cerdacului`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play balansul_cerdacului --seed 1`. Sursa implementării: `polytabletop-games/src/c4_stapel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Pui piese de greutate 1–4 pe taler stâng (s) sau drept (d). Dacă diferența dintre talere depășește 5, iei înapoi piesele talerului mai greu. Primul fără piese câștigă. Fără șansă: totul e determinist.

Mutări: pun w s|d.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
