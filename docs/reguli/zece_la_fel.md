# Zece la fel (`zece_la_fel`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zece_la_fel --seed 1`. Sursa implementării: `polytabletop-games/src/zr_duel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: fiecare are 10 zaruri. Pe rând arunci zarurile neblocate: la prima aruncare alegi fața-țintă („tinta v”), apoi blochezi zarurile cu fața-țintă (“bloc”) sau schimbi ținta („schimb v”) dacă altă față are mai multe decât ai blocat. Primul care blochează toate 10 câștigă (celălalt are o tură de egalare). Mutări: tira, tinta v, bloc, schimb v.

Detalii: La schimbarea țintei zarurile blocate anterior se eliberează.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Ten Alike.
