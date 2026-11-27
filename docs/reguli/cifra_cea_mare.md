# Cifra cea mare (`cifra_cea_mare`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cifra_cea_mare --seed 1`. Sursa implementării: `polytabletop-games/src/zr_duel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: în fiecare rundă cine începe alege 2–4 zaruri, le aruncă și le aranjează într-un număr; celălalt poate renunța (dă 1 punct) sau arunca același număr de zaruri. Cel cu numărul mai mare ia (zaruri−1) puncte. 6 runde. Mutări: zaruri n, tira, ordin cifre, primesc, renunt.

Detalii: AI-ul calculează exact probabilitatea de a fi depășit pentru fiecare număr.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Big Number.
