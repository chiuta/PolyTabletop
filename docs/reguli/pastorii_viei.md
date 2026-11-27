# Păstorii viei (duel: sat) (`pastorii_viei`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pastorii_viei --seed 1`. Sursa implementării: `polytabletop-games/src/pastorii_viei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în sat, zaruri d6. Fiecare alege o clasă (alege k): Cioară de seară, Alchimist, Pavăzar. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 dreptate); special costă dreptate și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: garda întoarce 1 daună atacatorului; o lovitură de cel puțin 3 daune provoacă sângerare (1 PV pe tură, cel mult 3 ture). Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Cioară de seară (PV 12, 2×d6 ≥4, daune 2, armură 1, «sete»: vindeci jumătate din daune, cost 2); Alchimist (PV 10, 2×d6 ≥4, daune 2, armură 0, «spori»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Pavăzar (PV 14, 2×d6 ≥4, daune 2, armură 1, «atac de pavăză»: +2 daune pe reușită, cost 2). Resursa se numește «dreptate» (+1 pe tură, maximum 5). Poțiuni: 1.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Shepherds of the Vine (duel: village).
