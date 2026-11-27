# Popoarele și epocile (`popoarele_si_epocile`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play popoarele_si_epocile --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 5 zaruri; ogoare (impar), fierărie (par), armată (≥3, costă hrană) și monument (orice, costă oțel). Fiecare loc revine celui cu suma mai mare la final de rundă (egal = nimeni). Hrana, oțelul și gloria se convertesc în puncte (3/2/1). 6 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Venit de 1 hrană pe rundă pentru fiecare.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Peoples and Epochs.
