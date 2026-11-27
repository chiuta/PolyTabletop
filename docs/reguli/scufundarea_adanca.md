# Scufundarea adâncă (`scufundarea_adanca`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play scufundarea_adanca --seed 1`. Sursa implementării: `polytabletop-games/src/zr_misc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: doi scafandri cu 25 de aer comun, pe o cărare de 16 comori. La începutul turei alegi „coboara” sau „urca” (o singură întoarcere), aerul scade cu comorile purtate, arunci 2 zaruri de 1–3 minus comorile purtate și mergi (sari peste locuri ocupate), apoi „iau”, „las” sau „nimic”. Cine nu ajunge înapoi când se termină aerul pierde comorile. 3 runde. Mutări: coboara, urca, tira, iau, las, nimic.

Detalii: Valorile comorilor sunt publice.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Deep Dive.
