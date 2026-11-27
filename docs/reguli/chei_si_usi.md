# Cheile și Ușile (`chei_si_usi`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chei_si_usi --seed 1`. Sursa implementării: `polytabletop-games/src/c4_memo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> 8 perechi de chei, lacăte, uși și lăzi cu fața în jos. Întorci două cărți (intorc i, arata); pereche găsită: +1 și mai întorci. Cele mai multe perechi câștigă.

Mutări: intorc i, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
