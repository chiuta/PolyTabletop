# Peritorii coroanei (`peritorii_coroanei`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play peritorii_coroanei --seed 1`. Sursa implementării: `polytabletop-games/src/zr_risc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri; la fiecare aruncare alegi o valoare neluată în tura asta și păstrezi toate zarurile cu ea (valoare × număr). Dacă nicio valoare nouă nu ți se potrivește, tura e ratată. Toate cele șase valori adunate dau +8. Poți opri oricând și încasezi. 5 runde. Mutări: tira, pastrez v, continui, opresc.

Detalii: Un joc de colecționat valori distincte: fiecare valoare se poate lua o singură dată pe tură.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Suitors of the Crown.
