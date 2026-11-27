# Camera Somnoroasă (`camara_adormita`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play camara_adormita --seed 1`. Sursa implementării: `polytabletop-games/src/c4_capcana.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Furi gustări din bucătărie fără să trezești tatăl: fiecare acțiune costă ticuri (1 sau 2) și dă 1 sau 3 gustări. Pragul ascuns e între 4 și 10; dacă îl depășești pierzi gustările din tură, cu stop le păstrezi. 8 ture, cel mai mare scor câștigă.

Mutări: act k, stop, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
