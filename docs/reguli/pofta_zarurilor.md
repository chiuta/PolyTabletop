# Pofta zarurilor (`pofta_zarurilor`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pofta_zarurilor --seed 1`. Sursa implementării: `polytabletop-games/src/zr_risc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri; punctează fiecare 1 (10), fiecare 5 (5) și grupurile de trei sau mai multe (1: 100, celelalte valoare×10, dublu pentru fiecare zar în plus). Păstrezi cel puțin un zar care punctează; dacă niciunul nu punctează, pierzi tura. Dacă toate cele 6 punctează, arunci iar toate. Încasezi cu cel puțin 15 puncte. 6 runde. Mutări: tira, pastrez 1x2 5x1 …, continui, opresc.

Detalii: Selecția „pastrez” enumeră toate combinațiile în care fiecare zar păstrat punctează.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Dice Hunger.
