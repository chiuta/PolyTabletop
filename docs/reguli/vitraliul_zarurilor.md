# Vitraliul zarurilor (`vitraliul_zarurilor`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vitraliul_zarurilor --seed 1`. Sursa implementării: `polytabletop-games/src/zr_grid.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: tablă 4×4; 5 zaruri colorate comune pe rundă. Primul zar la margine, apoi mereu lipit (și în diagonală) de unul existent; vecinii pe orizontală/verticală nu pot avea aceeași culoare sau valoare. Puncte: zaruri puse, rânduri cu 4 valori distincte (4), coloane cu 4 culori distincte (5). 4 runde. Mutări: tira, pon d r c, salt d.

Detalii: Zarul rămas la sfârșitul rundei se pierde.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Dice Stained Glass.
