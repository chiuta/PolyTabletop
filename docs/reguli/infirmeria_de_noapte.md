# Infirmeria de noapte (`infirmeria_de_noapte`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play infirmeria_de_noapte --seed 1`. Sursa implementării: `polytabletop-games/src/zr_flota.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: două zaruri pe rundă; paturile și cabinetele cu numărul aruncat aduc fonduri sau vindecări. Cumperi dotări cu fonduri (3 exemplare fiecare). Scor: puncte + vindecări/2. 8 runde. Mutări: tira, compro i, pas.

Detalii: Voluntarii și cabinetul produc și la tura adversarului.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Night Infirmary.
