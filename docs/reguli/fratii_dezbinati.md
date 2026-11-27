# Frații dezbinați (bătălie: fortăreață) (`fratii_dezbinati`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fratii_dezbinati --seed 1`. Sursa implementării: `polytabletop-games/src/fratii_dezbinati.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: bătălie de miniaturi pe tablă hexagonală 7×7 (fortăreață), 6 unități pe parte (Pușcași blindați, Trăgători grei, Cuirasat, Tun de asediu, Căpitan); teren: # blocaj, + acoperire, ~ teren greu, * obiectiv (2 obiectiv(e) «*»). 6 runde; fiecare unitate se activează o dată pe rundă, alternativ: «uN r,c» mută unitatea N, «uN r,c x uM» mută și atacă. Atacul aruncă zaruri d6 (lovește la ≥ tn), apoi salvări; acoperirea ușurează salvarea. Reguli proprii: tir din mișcare: trăgătorii care s-au mutat au −1 zar; flancare: +1 zar dacă un aliat e lângă țintă; moral: o unitate la cel mult o treime din PV șovăie (−1 zar). Puncte: pierderile provocate (după cost) + 2 pe obiectiv controlat la sfârșitul rundei; câștigi eliminând armata adversă sau cu mai multe puncte la final. Mutări: uN r,c, uN r,c x uM, tira.

Detalii: Unități: Pușcași blindați (PV 4+, mișcare 3, rază 1, zaruri 3, salvare 5, cost 4); Trăgători grei (PV 3+, mișcare 3, rază 4, zaruri 3, salvare -, cost 4); Cuirasat (PV 6+, mișcare 3, rază 1, zaruri 4, salvare 4, cost 8); Tun de asediu (PV 2+, mișcare 2, rază 6, zaruri 2, salvare -, cost 5); Căpitan (PV 3+, mișcare 4, rază 1, zaruri 3, salvare 5, cost 5). Valorile exacte sunt afișate în starea jocului.

Motor „bătălie de miniaturi” (t2_mini). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Divided Brothers (battle: fortress).
