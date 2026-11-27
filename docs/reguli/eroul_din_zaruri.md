# Eroul din zaruri (`eroul_din_zaruri`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play eroul_din_zaruri --seed 1`. Sursa implementării: `polytabletop-games/src/zr_grid.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: patru atribute cu câte trei locuri; 4 zaruri comune pe rundă, două alegeri fiecare. Pui zarul pe un atribut („pon d a”). Un atribut complet dă ponderea (3,2,2,1) × modificator al sumei (≤6: −1, 7–9: 0, 10–12: +1, 13–15: +2, 16+: +3) + 2. 6 runde. Mutări: tira, pon d a, salt d.

Detalii: Atributele incomplete dau câte 1 punct de progres.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Hero of Dice.
