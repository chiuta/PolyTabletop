# Schițe de construcție (`schite_de_constructie`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play schite_de_constructie --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 7 zaruri pe rundă; patru schițe cer câte trei zaruri de același tip: turn (≥5), pod (≤2), fântână (par), casă (impar). Primul care adună trei pe o schiță o termină și încasează puncte/resurse. 5 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Fiecare schiță acceptă maxim trei zaruri ale aceluiași jucător pe rundă.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Building Sketches.
