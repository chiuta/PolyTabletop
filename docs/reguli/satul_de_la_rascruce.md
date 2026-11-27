# Satul de la răscruce (`satul_de_la_rascruce`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play satul_de_la_rascruce --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 5 zaruri aruncate la începutul rundei; le pui pe clădiri cu condiții (bancă ≥4, moară impar, han par, primărie orice, fântână ≤3). La final de rundă fiecare clădire revine celui cu suma mai mare pe ea (egal = nimeni). 6 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Aurul valorează 1 punct la 3, grâul 1 la 2.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Crossroads Village.
