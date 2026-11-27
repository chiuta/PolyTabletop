# Petale și spini (`petale_si_spini`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play petale_si_spini --seed 1`. Sursa implementării: `polytabletop-games/src/zr_duel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de ghicit regula: cine aruncă alege 3–5 zaruri; celălalt ghicește „numărul petalelor” (ghic k) sau trece. Corect = zaruri−2 puncte pentru ghicitor, greșit = 1 punct pentru cel care aruncă. După runda 3 regula se dezvăluie în afișare. 8 runde. Mutări: zaruri n, tira, ghic k, trec.

Detalii: Regula: zarul 3 valorează 2 petale, zarul 5 valorează 4, restul 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Petals and Thorns.
