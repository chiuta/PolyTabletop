# Zgârie-norul Cartonat (`turnul_rinocerului`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play turnul_rinocerului --seed 1`. Sursa implementării: `polytabletop-games/src/c4_stapel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Construiți un turn comun din pereți cartonați de 3 înălțimi. Fiecare piesă pusă (pun w) are un risc public de prăbușire care crește cu înălțimea; la verific se hotărăște căderea. Cine dărâmă ia înapoi piesele căzute. Câștigă primul care rămâne fără piese.

Mutări: pun w, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
