# Păianjenul de pe Fir (`paianjen_pe_fir`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play paianjen_pe_fir --seed 1`. Sursa implementării: `polytabletop-games/src/c4_cursa.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Cursă de 20 de pătrate pe un fir; trebuie să ajungi exact la capăt. Fața păianjenului te trage înapoi 4 pătrate; scurtături 5→9 și 13→16; poți lovi pionul adversarului. Primul la capăt câștigă.

Mutări: zar, mut k f, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
