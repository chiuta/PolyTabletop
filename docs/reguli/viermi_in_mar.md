# Viermele din Măr (`viermi_in_mar`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play viermi_in_mar --seed 1`. Sursa implementării: `polytabletop-games/src/c4_cursa.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cursă de 18 pătrate cu 2 viermi pe jucător. Fața specială te lasă să arunci din nou; o gaură la 9 te întoarce la start; scurtătură de la 4 la 8; poți lovi viermele adversarului. Primul cu ambii viermi la capăt câștigă.

Mutări: zar, mut k f, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
