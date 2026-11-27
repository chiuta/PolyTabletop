# Drumuri și șine (`drumuri_si_sine`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play drumuri_si_sine --seed 1`. Sursa implementării: `polytabletop-games/src/zr_grid.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: tablă 5×5 cu o răscruce în centru; 4 zaruri de piese (dreaptă, curbă, T, cruce) pe rundă, 2 alegeri fiecare. Piesa se pune cu rotație aleasă lângă o capătă deschisă a rețelei, cu o legătură înapoi. Scor: 2 × piese conectate la centru − capete moarte. 7 runde. Mutări: tira, pon d r c rot, salt d.

Detalii: Capetele deschise spre margine sau spre goluri sunt „capete moarte”.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Roads and Rails.
