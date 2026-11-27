# Zarul nebuniei (`zarul_nebuniei`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zarul_nebuniei --seed 1`. Sursa implementării: `polytabletop-games/src/zr_risc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: fiecare pornește cu 3 puncte de luciditate (maxim 4). Pe rând arunci un zar cu 12 fețe: marele zeu (cel mai lucid pierde 1), tentacul (pierzi 1), semn (alegi: lovesc = adversarul pierde 1, sau vindec = +1), ochi (cel mai puțin lucid pierde 1), soare (+1). La 0 pierzi; după 80 de ture decide luciditatea. Mutări: tira, lovesc, vindec.

Detalii: Doar fața „semn” oferă o alegere; restul sunt efecte automate publice.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Die of Madness.
