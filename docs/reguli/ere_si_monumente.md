# Ere și monumente (`ere_si_monumente`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ere_si_monumente --seed 1`. Sursa implementării: `polytabletop-games/src/zr_trei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 5 zaruri, până la 3 aruncări. Muncă, hrană, mărfuri, monede, craniu (nu se relansează; 2 craniu = −1, 3 = pierzi muncitorii). 6 muncitori = monument (+4), 3 mărfuri = +2, hrană 2 pe tură (lipsa costă). 8 ture. Mutări: tira, relansez i,j, fin.

Detalii: Fiecare jucător are 8 ture.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Ages and Monuments.
