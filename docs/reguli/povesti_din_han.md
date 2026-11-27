# Povești din han (duel: sat) (`povesti_din_han`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play povesti_din_han --seed 1`. Sursa implementării: `polytabletop-games/src/povesti_din_han.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în sat, zaruri d6. Fiecare alege o clasă (alege k): Pândar, Pavăzar, Hipnotizor. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 poftă); special costă poftă și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: garda întoarce 1 daună atacatorului; «lovesc+» cheltuiește 1 poftă pentru un zar în plus. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Pândar (PV 10, 3×d6 ≥4, daune 1, armură 0, «săgeată ascuțită»: ignoră armura, cost 2); Pavăzar (PV 13, 2×d6 ≥4, daune 2, armură 1, «lovitură zdrobitoare»: +2 daune pe reușită, cost 2); Hipnotizor (PV 10, 2×d6 ≥4, daune 2, armură 1, «laț»: dacă lovește, adversarul pierde tura, cost 2). Resursa se numește «poftă» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Tales from the Inn (duel: village).
