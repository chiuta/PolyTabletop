# Harta cu cifre (`harta_cu_cifre`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play harta_cu_cifre --seed 1`. Sursa implementării: `polytabletop-games/src/zr_grid.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: tablă 4×4; 4 zaruri comune pe rundă, două alegeri fiecare. Pui valoarea într-o celulă liberă astfel încât toate rândurile și coloanele să rămână crescătoare (necrescător, fără scăderi). Puncte: celule, rânduri/coloane complete (3). 8 runde. Mutări: tira, pon d r c, salt d.

Detalii: Se verifică toate celulele ocupate din același rând/coloană, nu doar vecinele.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Map of Numbers.
