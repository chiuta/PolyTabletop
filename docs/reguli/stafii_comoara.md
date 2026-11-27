# Stafiile și Comoara (`stafii_comoara`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play stafii_comoara --seed 1`. Sursa implementării: `polytabletop-games/src/c4_memo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> 7 perechi de comori și 3 stafii cu fața în jos. Pereche găsită: +1 și mai întorci. Stafie: adversarul ia un punct și tura trece. Cel mai mare scor câștigă când se termină comorile.

Mutări: intorc i, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
