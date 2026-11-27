# Șantierul orbital (`santierul_orbital`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play santierul_orbital --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 4 nave-zaruri pe rundă, locuri exclusive cu efect imediat: docuri de minereu (≤3) și combustibil (≥4), uzină (par, costă 2 minereu + 1 combustibil), stație (impar), colonizare (6, costă 3 minereu + 2 combustibil, 2 puncte). Venit 1 minereu pe rundă. 8 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Coloniile finale valorează puncte; resursele nu.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Orbital Shipyard.
