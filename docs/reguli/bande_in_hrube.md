# Bande în hrube (bătălie: oraș) (`bande_in_hrube`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bande_in_hrube --seed 1`. Sursa implementării: `polytabletop-games/src/bande_in_hrube.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: bătălie de miniaturi pe tablă pătrățică 7×7 (oraș), 5 unități pe parte (Pistolar, Aruncător de grenade, Șef de bandă, Mutant uriaș); teren: # blocaj, + acoperire, ~ teren greu, * obiectiv (2 obiectiv(e) «*»). 7 runde; fiecare unitate se activează o dată pe rundă, alternativ: «uN r,c» mută unitatea N, «uN r,c x uM» mută și atacă. Atacul aruncă zaruri d6 (lovește la ≥ tn), apoi salvări; acoperirea ușurează salvarea. Reguli proprii: mișcare și distanțe cu diagonale; moral: o unitate la cel mult o treime din PV șovăie (−1 zar); contralovitură: apărătorul de corp la corp răspunde cu jumătate din zaruri. Puncte: pierderile provocate (după cost) + 2 pe obiectiv controlat la sfârșitul rundei; câștigi eliminând armata adversă sau cu mai multe puncte la final. Mutări: uN r,c, uN r,c x uM, tira.

Detalii: Unități: Pistolar (PV 4+, mișcare 3, rază 1, zaruri 3, salvare 5, cost 4); Aruncător de grenade (PV 3+, mișcare 3, rază 4, zaruri 3, salvare -, cost 4); Șef de bandă (PV 3+, mișcare 4, rază 1, zaruri 3, salvare 5, cost 5); Mutant uriaș (PV 6+, mișcare 3, rază 1, zaruri 4, salvare 4, cost 8). Valorile exacte sunt afișate în starea jocului.

Motor „bătălie de miniaturi” (t2_mini). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Gangs in the Undercroft (battle: city).
