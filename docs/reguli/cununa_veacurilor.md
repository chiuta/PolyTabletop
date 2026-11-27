# Cununa veacurilor (duel: oraș) (`cununa_veacurilor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cununa_veacurilor --seed 1`. Sursa implementării: `polytabletop-games/src/cununa_veacurilor.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în oraș, zaruri d20. Fiecare alege o clasă (alege k): Alchimist, Ciomăgar, Vraci. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 faimă); special costă faimă și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: dacă toate zarurile (cel puțin două) reușesc, daunele se dublează; «lovesc+» cheltuiește 1 faimă pentru un zar în plus. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Alchimist (PV 9, 1×d20 ≥11, daune 4, armură 0, «otravă lentă»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Ciomăgar (PV 14, 1×d20 ≥11, daune 4, armură 1, «lovitură zdrobitoare»: +2 daune pe reușită, cost 2); Vraci (PV 12, 1×d20 ≥11, daune 2, armură 2, «leac»: vindecă 4 PV, cost 2). Resursa se numește «faimă» (+1 pe tură, maximum 5). Poțiuni: 1.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Crown of Ages (duel: city).
