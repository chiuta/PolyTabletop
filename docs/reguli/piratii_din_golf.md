# Pirații din Golf (`piratii_din_golf`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play piratii_din_golf --seed 1`. Sursa implementării: `polytabletop-games/src/c4_insula.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Inel de 6 câmpuri numerotate (lemn, aur, lână) cu 6 locuri de ancorare între ele. La zar câmpul cu numărul aruncat produce pentru ancorele vecine. Construiești o ancoră cu 1 lemn, 1 aur, 1 lână; schimbi 2 la 1. Primul cu 4 ancore câștigă.

Mutări: zar, ancor s, schimb a b, pas.

Observații: norocul (zar, extragere, capcană ascunsă) se hotărăște la mutarea forțată, cu un zar re-însămânțat la clonare, deci calculatorul nu îl poate prevedea; AI-ul are adâncime fixă (fără ceas) și nu citește informația ascunsă. Jocurile de îndemânare sunt modelate ca risc public.
