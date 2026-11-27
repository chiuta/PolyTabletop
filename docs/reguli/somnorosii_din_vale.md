# Somnoroșii din vale (duel: noapte) (`somnorosii_din_vale`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play somnorosii_din_vale --seed 1`. Sursa implementării: `polytabletop-games/src/somnorosii_din_vale.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în noapte, zaruri d12. Fiecare alege o clasă (alege k): Păstor, Luptător cu laț, Vrăjitoare de râu. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 vis); special costă vis și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: «lovesc+» cheltuiește 1 vis pentru un zar în plus; garda întoarce 1 daună atacatorului. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Păstor (PV 11, 2×d12 ≥7, daune 1, armură 2, «apă sfințită»: vindecă 4 PV, cost 2); Luptător cu laț (PV 11, 2×d12 ≥7, daune 2, armură 1, «laț»: dacă lovește, adversarul pierde tura, cost 2); Vrăjitoare de râu (PV 9, 2×d12 ≥7, daune 2, armură 0, «praf de ochi»: otravă 2: 1 PV pe tură, 2 ture, cost 2). Resursa se numește «vis» (+1 pe tură, maximum 5). Poțiuni: 1.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Sleepers of the Vale (duel: night).
