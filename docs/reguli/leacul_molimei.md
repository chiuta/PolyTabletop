# Leacul molimei (`leacul_molimei`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play leacul_molimei --seed 1`. Sursa implementării: `polytabletop-games/src/zr_trei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de întrecere: 5 zaruri (patru boli, pericol, medic-joker), până la 3 aruncări; pericolul nu se relansează. Trei zaruri la fel (cu medici) dau o probă (cinci = două); trei probe dintr-o boală = leac. Trei pericole ți-o iau pe una. Primul cu 3 leacuri câștigă. Mutări: tira, relansez i,j, fin.

Detalii: Medicul completează orice boală.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Cure for the Plague.
