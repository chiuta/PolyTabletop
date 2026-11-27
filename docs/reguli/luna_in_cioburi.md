# Luna în cioburi (duel: noapte) (`luna_in_cioburi`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play luna_in_cioburi --seed 1`. Sursa implementării: `polytabletop-games/src/luna_in_cioburi.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în noapte, zaruri d10. Fiecare alege o clasă (alege k): Frate pavăzar, Pavăzar, Fermecător. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 cioburi); special costă cioburi și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: o lovitură de cel puțin 3 daune provoacă sângerare (1 PV pe tură, cel mult 3 ture); «lovesc+» cheltuiește 1 cioburi pentru un zar în plus. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Frate pavăzar (PV 13, 2×d10 ≥6, daune 1, armură 2, «încurajare»: vindecă 4 PV, cost 2); Pavăzar (PV 15, 2×d10 ≥6, daune 2, armură 1, «lovitură zdrobitoare»: +2 daune pe reușită, cost 2); Fermecător (PV 9, 2×d10 ≥6, daune 2, armură 0, «spori»: otravă 2: 1 PV pe tură, 2 ture, cost 2). Resursa se numește «cioburi» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Shattered Moon (duel: night).
