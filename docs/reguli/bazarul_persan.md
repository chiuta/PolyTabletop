# Bazarul persan (`bazarul_persan`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bazarul_persan --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri; fiecare taraba (1–6) acceptă o valoare exactă și un singur zar pe rundă; mai sunt caravana și hanul. Efectul e imediat (monede, mărfuri, mirodenii; unele locuri costă). La final: 4 monede, 2 mărfuri sau 1 mirodenie = 1 punct. 6 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Locurile sunt exclusive: primul care le ocupă blochează adversarul.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Persian Bazaar.
