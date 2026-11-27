# Vulpea Dibace (`vulpea_dibace`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vulpea_dibace --seed 1`. Sursa implementării: `polytabletop-games/src/c4_pista.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Un vulpoi ascuns dintre 12 suspecți (culoare, pălărie, coadă); nimeni nu-l știe. Pe rând puneți întrebări publice (intreb A=v, cel mult 3 pe jucător) și ghiciți (ghic k). Ghicit corect +2, greșit +1 pentru celălalt; 4 runde.

Mutări: intreb A=v, ghic k, raspuns, arata, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
