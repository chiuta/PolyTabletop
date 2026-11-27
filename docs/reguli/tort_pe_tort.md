# Tortul cu Straturi (`tort_pe_tort`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tort_pe_tort --seed 1`. Sursa implementării: `polytabletop-games/src/c4_stapel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Puneți pe rând straturi de tort (1 cireașă, 2 blat mic, 3 blat mijlociu, 4 blat mare). Un strat mai mare peste unul mai mic e mai riscant. Zarul hotărăște câte piese pui. Cine prăbușește tortul (verific) ia piesele căzute. Primul fără piese câștigă.

Mutări: zar, pun w, ofer w, verific.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
