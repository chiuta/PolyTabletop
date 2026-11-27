# Verde, galben, roșu (`verde_galben_rosu`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play verde_galben_rosu --seed 1`. Sursa implementării: `polytabletop-games/src/zr_risc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 10 zaruri cu 3 fețe verzi, 2 galbene, 1 roșie. Fiecare verde aruncat se păstrează (1 punct), galbenele se pot relansa, roșul scoate zarul din joc; dacă apare roșu fără niciun verde, pierzi punctele turei. Poți opri oricând după o aruncare și încasezi. 6 runde. Mutări: tira, continui, opresc.

Detalii: Dacă toate zarurile ies din joc, câștigi automat punctele turei. Probabilitatea de ratare crește odată cu scăderea numărului de zaruri rămase.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Green, Yellow, Red.
