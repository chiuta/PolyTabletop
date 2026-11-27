# Ținta fixată (`tinta_fixata`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tinta_fixata --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri pe rundă; șase cărți-țintă cer fiecare două zaruri cu aceeași valoare (1 la 6), cu 5,4,3,3,2,1 puncte. Cine pune al doilea zar ia cartea. 8 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Cărțile se reînnoiesc la fiecare rundă.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Fixed Target.
