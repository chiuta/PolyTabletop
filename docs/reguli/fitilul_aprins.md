# Fitilul aprins (`fitilul_aprins`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fitilul_aprins --seed 1`. Sursa implementării: `polytabletop-games/src/zr_misc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: patru bombe cu 4–6 fire, fiecare cu o condiție de zar. Pe rând arunci 5 zaruri și tai fire („des v fir”); cine taie ultimul fir ia valoarea bombei. Fiecare „pas” consumă un fitil (5); la zero bomba explodează și cel care a spus pasul pierde 2. Mutări: tira, des v fir, pas.

Detalii: Cine lasă un singur fir liber îi oferă adversarului bomba.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Lit Fuse.
