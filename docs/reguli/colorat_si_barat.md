# Colorat și bărat (`colorat_si_barat`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play colorat_si_barat --seed 1`. Sursa implementării: `polytabletop-games/src/zr_grid.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: tablă 5×5 cu cinci regiuni; 4 zaruri comune (culoare + număr 1–5). Alegi un zar și barezi „număr” celule consecutive pe un rând din regiunea culorii („pon d r c”), legate de ce ai bărat (prima dată prin coloana din mijloc). Puncte: celule, coloane complete (4, sau 2 dacă și adversarul a completat-o), regiuni complete (3). 8 runde. Mutări: tira, pon d r c, salt d.

Detalii: Harta regiunilor e afișată în joc.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Colour and Cross.
