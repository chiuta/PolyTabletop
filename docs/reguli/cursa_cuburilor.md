# Cursa cuburilor (`cursa_cuburilor`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cursa_cuburilor --seed 1`. Sursa implementării: `polytabletop-games/src/zr_misc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: fiecare are un săculeț cu zaruri (la început 5 simple). Tura: extragi 3 zaruri și le arunci (fiecare față dă mișcare și monede); poți trage încă unul („trag”) dar cu două fețe goale ratezi tura. Apoi cumperi un zar mai bun (alergător 4, negustor 3, turbo 6). Pista are 28 de pași; ultima tură se joacă egal. Mutări: tira, trag, opresc, cumpar tip, pas.

Detalii: Compoziția săculețului e publică; ordinea extragerii e aleatoare.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Race of the Cubes.
