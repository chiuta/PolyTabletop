# Farfurii zburătoare (`farfurii_zburatoare`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play farfurii_zburatoare --seed 1`. Sursa implementării: `polytabletop-games/src/zr_risc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 10 zaruri cu tancuri, raze și trei feluri de pământeni. La fiecare aruncare tancurile se pun deoparte, apoi alegi UN fel de față neluat încă și păstrezi toate zarurile cu el. Te poți opri doar dacă ai cel puțin tot atâtea raze ca tancuri; altfel pierzi tura. Punctaj: pământenii păstrați (+3 dacă ai toate cele trei feluri). 6 runde. Mutări: tira, pastrez raza|om|vaca|gaina, continui, opresc.

Detalii: Fețele zarului: tanc, rază (de două ori), om, vacă, găină. Tancurile aruncate se adaugă la „armata” ta și nu mai pot fi relansate; razele păstrate sunt scutul. Dacă la o aruncare nu mai ai niciun fel neales, tura e ratată.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Flying Saucers.
