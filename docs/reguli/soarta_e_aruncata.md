# Soarta e aruncată (`soarta_e_aruncata`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play soarta_e_aruncata --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri pe rundă; fiecare clădire cere două zaruri cu aceeași valoare (forum 1, terme 2, apeduct 3, arenă 4, templu 5, drum 6). Primul care pune al doilea zar o închide și încasează recompensa. 6 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Zarurile puse de adversar pe un loc închis dispar în runda aceea.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Die Is Cast.
