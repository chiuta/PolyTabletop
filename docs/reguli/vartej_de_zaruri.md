# Vârtej de zaruri (`vartej_de_zaruri`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vartej_de_zaruri --seed 1`. Sursa implementării: `polytabletop-games/src/zr_duel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: fiecare are 8 zaruri. Trei grămezi comune au vârfuri vizibile. Pe rând arunci zarurile rămase și pui oricâte pe o grămadă al cărei vârf diferă cu cel mult 1 („joc v g”), apoi „pas”. Primul care rămâne fără zaruri câștigă. Mutări: tira, joc v g, pas.

Detalii: Pasul termină tura; zarurile nepuse se aruncă din nou la tura următoare.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Dice Whirl.
