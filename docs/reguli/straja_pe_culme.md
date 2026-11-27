# Strajă pe culme (bătălie: munte) (`straja_pe_culme`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play straja_pe_culme --seed 1`. Sursa implementării: `polytabletop-games/src/straja_pe_culme.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: bătălie de miniaturi pe tablă pătrățică 8×7 (munte), 6 unități pe parte (Străji, Arcași de potecă, Căpetenie, Iscoade călare); teren: # blocaj, + acoperire, ~ teren greu, * obiectiv (2 obiectiv(e) «*»). 7 runde; fiecare unitate se activează o dată pe rundă, alternativ: «uN r,c» mută unitatea N, «uN r,c x uM» mută și atacă. Atacul aruncă zaruri d6 (lovește la ≥ tn), apoi salvări; acoperirea ușurează salvarea. Reguli proprii: mișcare și distanțe cu diagonale; contralovitură: apărătorul de corp la corp răspunde cu jumătate din zaruri; asalt: unitățile de corp la corp care s-au mutat cel puțin 2 au +1 zar. Puncte: pierderile provocate (după cost) + 2 pe obiectiv controlat la sfârșitul rundei; câștigi eliminând armata adversă sau cu mai multe puncte la final. Mutări: uN r,c, uN r,c x uM, tira.

Detalii: Unități: Străji (PV 4+, mișcare 3, rază 1, zaruri 3, salvare 5, cost 4); Arcași de potecă (PV 3+, mișcare 3, rază 4, zaruri 3, salvare -, cost 4); Căpetenie (PV 3+, mișcare 4, rază 1, zaruri 3, salvare 5, cost 5); Iscoade călare (PV 4+, mișcare 5, rază 1, zaruri 3, salvare 5, cost 5). Valorile exacte sunt afișate în starea jocului.

Motor „bătălie de miniaturi” (t2_mini). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Watch on the Ridge (battle: mountain).
