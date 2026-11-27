# Insula cu Furtuni (`insula_cu_furtuni`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play insula_cu_furtuni --seed 1`. Sursa implementării: `polytabletop-games/src/c4_memo.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> 8 perechi de obiecte de pe insulă și 2 furtuni cu fața în jos. Pereche găsită: pionul tău înaintează 1–3. Furtună: te întoarce 2 pătrate. Primul la 15 câștigă.

Mutări: intorc i, arata.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
