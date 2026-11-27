# Baza stelară (`baza_stelara`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play baza_stelara --seed 1`. Sursa implementării: `polytabletop-games/src/zr_flota.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: în fiecare rundă se aruncă două zaruri; cărțile cu numărul egal cu suma produc pentru proprietar (cele pasive și la adversar). Apoi fiecare poate cumpăra o carte din piața publică (2 exemplare). Scor: puncte + energie/3. 10 runde. Mutări: tira, compro i, pas.

Detalii: Nu există cărți ascunse: piața e fixă și publică.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Star Outpost.
