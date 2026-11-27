# Zaruri în duel (`zaruri_in_duel`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zaruri_in_duel --seed 1`. Sursa implementării: `polytabletop-games/src/zr_duel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: fiecare are cinci zaruri (d4, d6, d8, d12, d20), aruncate la început. Atac de forță: un zar de-al tău cu valoare ≥ ținta o capturează („forta a t”); atac de îndemânare: suma a 2+ zaruri proprii = valoarea ținței („indemanare a,b t”). Atacatorii se re-aruncă. Câștigă cine rămâne cu zaruri sau, după două pasuri la rând, cu suma fețelor mai mare. Mutări: tira, forta, indemanare, pas.

Detalii: Zarurile capturate ies din joc; fețele zarurilor rămase decid la final.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Dice in Duel.
