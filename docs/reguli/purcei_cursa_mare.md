# Cursa Purceilor (`purcei_cursa_mare`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play purcei_cursa_mare --seed 1`. Sursa implementării: `polytabletop-games/src/c4_cursa.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cursă de 26 de pătrate cu 3 purcei pe jucător. Fața specială schimbă locul purceilor tăi cei mai avansați cu ai adversarului. Primul cu toți purceii la capăt câștigă.

Mutări: zar, mut k f, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
