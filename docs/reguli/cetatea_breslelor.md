# Cetatea breslelor (`cetatea_breslelor`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cetatea_breslelor --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri și 3 galbeni venit pe rundă; fiecare breaslă are condiție de valoare și cost per zar. La final de rundă fiecare breaslă revine celui cu suma mai mare (egal = nimeni). Puncte din bresle, galbeni (3=1p) și ucenici. 5 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Piața dă galbeni pentru rundele următoare.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en City of Guilds.
