# Orașul în creștere (`orasul_in_crestere`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play orasul_in_crestere --seed 1`. Sursa implementării: `polytabletop-games/src/zr_flota.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: două zaruri pe rundă; clădirile cu numărul egal cu UNUL dintre zaruri produc lemn sau faimă. Cumperi clădiri cu lemn (una pe rundă). Scor: puncte + faimă/2. 9 runde. Mutări: tira, compro i, pas.

Detalii: Aruncătorul alternează de la o rundă la alta.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Growing City.
