# Zar peste masă (`zar_peste_masa`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zar_peste_masa --seed 1`. Sursa implementării: `polytabletop-games/src/zr_arunc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de „aruncat peste masă” (îndemânarea e simulată cu probabilități publice): alegi ținta (usor 70% +1, tintit 50% +2, risc 30% +4; ratarea la risc dă 1 adversarului), adversarul poate folosi una din cele 2 gărzi (−20% șansă), apoi „tira”. Primul la 12 puncte câștigă. Mutări: usor, tintit, risc, garda, liber, tira.

Detalii: Gărzile sunt o resursă limitată (2 per jucător).

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Die Across the Table.
