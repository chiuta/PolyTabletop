# Fantoma Fugară (`fantoma_fugara`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fantoma_fugara --seed 1`. Sursa implementării: `polytabletop-games/src/c4_urmarire.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Un fantomă fuge pe un inel de 12 camere. Fiecare își mută urmăritorul (mut k înainte, inv k înapoi, 1–3); cine aterizează pe fantomă o prinde (+1). După fiecare mutare fantoma fuge (fuge). Primul la 3 prinderi câștigă.

Mutări: mut k, inv k, fuge.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
