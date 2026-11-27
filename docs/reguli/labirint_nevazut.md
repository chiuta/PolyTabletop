# Labirintul cu Pereți Nevăzuți (`labirint_nevazut`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play labirint_nevazut --seed 1`. Sursa implementării: `polytabletop-games/src/c4_labirint.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Tablă 5×5 cu pereți ascunși: fiecare muchie se hotărăște la prima încercare (30% perete), apoi rămâne cunoscută de amândoi. Mergi n/s/e/v; primul la comoara din centru câștigă.

Mutări: mut n|s|e|v, arata, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
