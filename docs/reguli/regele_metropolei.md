# Regele metropolei (`regele_metropolei`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play regele_metropolei --seed 1`. Sursa implementării: `polytabletop-games/src/zr_trei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 zaruri, până la 3 aruncări pe tură (relansez i,j). Fețele 1/2/3: trei la fel dau puncte; atac (lovești adversarul), vindecare (+viață, max 10), energie (2 energie = 1 punct). Primul la 20 de puncte sau care doboară adversarul câștigă. Mutări: tira, relansez i,j, fin.

Detalii: Fețele sunt afișate sub tablă.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en King of the Metropolis.
