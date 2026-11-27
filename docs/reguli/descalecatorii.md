# Descălecătorii (`descalecatorii`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play descalecatorii --seed 1`. Sursa implementării: `polytabletop-games/src/zr_flota.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: suma a două zaruri activează așezările cu acel număr (merinde sau pământ). Cumperi câmpuri, sate și fortărețe cu merinde. Scor: puncte + pământ/2. 9 runde. Mutări: tira, compro i, pas.

Detalii: Fântâna și drumul sunt pasive.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Settlers' Landing.
