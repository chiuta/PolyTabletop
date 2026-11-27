# Oastea de zaruri (`oastea_de_zaruri`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oastea_de_zaruri --seed 1`. Sursa implementării: `polytabletop-games/src/zr_trei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: fiecare armată are 8 unități (zaruri). Arunci toate unitățile (relansezi până la 2 ori): corp la corp, tragere, magie și perechi de semne lovesc; scuturile tale (aruncate la tura precedentă) apără; perechile de manevră recuperează unități. Pierzi dacă rămâi fără unități. Mutări: tira, relansez i,j, fin.

Detalii: Numărul de zaruri aruncate scade pe măsură ce pierzi unități.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Army of Dice.
