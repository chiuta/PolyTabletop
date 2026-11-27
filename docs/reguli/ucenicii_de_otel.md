# Ucenicii de oțel (duel: fier) (`ucenicii_de_otel`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ucenicii_de_otel --seed 1`. Sursa implementării: `polytabletop-games/src/ucenicii_de_otel.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în fier, zaruri d100. Fiecare alege o clasă (alege k): Ghicitor, Duh însetat, Spadasin. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 ritm); special costă ritm și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: o lovitură de cel puțin 3 daune provoacă sângerare (1 PV pe tură, cel mult 3 ture); dacă toate zarurile (cel puțin două) reușesc, daunele se dublează. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Ghicitor (PV 10, 1×d100 ≥51, daune 4, armură 0, «blestem»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Duh însetat (PV 12, 1×d100 ≥51, daune 4, armură 1, «sorb viața»: vindeci jumătate din daune, cost 2); Spadasin (PV 14, 1×d100 ≥51, daune 4, armură 1, «lovitură zdrobitoare»: +2 daune pe reușită, cost 2). Resursa se numește «ritm» (+1 pe tură, maximum 5). Poțiuni: 1.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Apprentices of Steel (duel: iron).
