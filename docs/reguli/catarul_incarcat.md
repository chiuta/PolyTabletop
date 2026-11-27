# Măgarul Încărcat (`catarul_incarcat`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play catarul_incarcat --seed 1`. Sursa implementării: `polytabletop-games/src/c4_stapel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Încărcați pe rând un măgar cu lopeți (1), găleți (2) și saci (3). Limita lui ascunsă e între 10 și 22; riscul public al fiecărei piese vine din ce se știe. La verific poate da cu copita: cine încarcă ia înapoi piesele căzute. Primul fără piese câștigă.

Mutări: pun w, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
