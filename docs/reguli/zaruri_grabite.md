# Zarurile Grăbite (`zaruri_grabite`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zaruri_grabite --seed 1`. Sursa implementării: `polytabletop-games/src/c4_cursa.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cursă de 28 de pătrate cu 2 pioni pe jucător. Arunci două zaruri și alegi unul pentru un pion (mut k f); fața 0 nu există, dar o capcană la 14 te întoarce la start și o scurtătură duce de la 8 la 12. Primul cu ambii pioni la capăt câștigă.

Mutări: zar, mut k f, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
