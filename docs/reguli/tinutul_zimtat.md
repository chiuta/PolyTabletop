# Ținutul zimțat (duel: munte) (`tinutul_zimtat`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tinutul_zimtat --seed 1`. Sursa implementării: `polytabletop-games/src/tinutul_zimtat.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în munte, zaruri d20. Fiecare alege o clasă (alege k): Căpcăun flămând, Ghicitor, Mercenar. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 mânie); special costă mânie și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: sub jumătate din PV ai +1 daună; garda întoarce 1 daună atacatorului. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Căpcăun flămând (PV 12, 1×d20 ≥11, daune 4, armură 1, «sete»: vindeci jumătate din daune, cost 2); Ghicitor (PV 10, 1×d20 ≥11, daune 4, armură 0, «blestem»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Mercenar (PV 13, 1×d20 ≥11, daune 4, armură 1, «lovitură zdrobitoare»: +2 daune pe reușită, cost 2). Resursa se numește «mânie» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Jagged Land (duel: mountain).
