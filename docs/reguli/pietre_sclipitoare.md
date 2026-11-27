# Pietrele Sclipitoare (`pietre_sclipitoare`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pietre_sclipitoare --seed 1`. Sursa implementării: `polytabletop-games/src/c4_capcana.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Tragi pietre dintr-un sac cu compoziție publică (valori 1–4 și 3 pietre cenușii). O singură piatră cenușie îți ia potul. Cu stop îl bagi în scor. 8 ture; cel mai mare scor câștigă.

Mutări: tragi, stop, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
