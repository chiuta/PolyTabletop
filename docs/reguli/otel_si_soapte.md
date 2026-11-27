# Oțel și șoapte (duel: turn) (`otel_si_soapte`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play otel_si_soapte --seed 1`. Sursa implementării: `polytabletop-games/src/otel_si_soapte.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în turn, zaruri d20. Fiecare alege o clasă (alege k): Doctor de câmp, Vânător, Paznic de nopți. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 vrajă); special costă vrajă și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: sub jumătate din PV ai +1 daună; «lovesc+» cheltuiește 1 vrajă pentru un zar în plus. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Doctor de câmp (PV 11, 1×d20 ≥11, daune 2, armură 2, «bandaj»: vindecă 4 PV, cost 2); Vânător (PV 11, 2×d20 ≥10, daune 2, armură 0, «săgeată ascuțită»: ignoră armura, cost 2); Paznic de nopți (PV 12, 1×d20 ≥11, daune 4, armură 1, «privire fixă»: dacă lovește, adversarul pierde tura, cost 2). Resursa se numește «vrajă» (+1 pe tură, maximum 5). Poțiuni: 1.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Steel and Whispers (duel: tower).
