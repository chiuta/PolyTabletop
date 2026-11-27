# Ouăle din Cuib (`oua_din_cuib`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oua_din_cuib --seed 1`. Sursa implementării: `polytabletop-games/src/c4_capcana.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Tragi dintr-un sac cu compoziție publică: ouă de valoare 1–3 le adaugi în pot, o piatră rea te costă; la 2 pietre pierzi potul. Cu stop bagi potul în scor. 8 ture în total; cel mai mare scor câștigă.

Mutări: tragi, stop, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
