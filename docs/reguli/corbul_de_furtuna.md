# Corbul de furtună (duel: vânt) (`corbul_de_furtuna`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play corbul_de_furtuna --seed 1`. Sursa implementării: `polytabletop-games/src/corbul_de_furtuna.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în vânt, zaruri d6. Fiecare alege o clasă (alege k): Maestru de lanțuri, Șoptitor, Tâlhar cu pumnal. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 libertate); special costă libertate și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: «lovesc+» cheltuiește 1 libertate pentru un zar în plus; dacă toate zarurile (cel puțin două) reușesc, daunele se dublează. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Maestru de lanțuri (PV 10, 2×d6 ≥4, daune 2, armură 1, «bubuitură de clopot»: dacă lovește, adversarul pierde tura, cost 2); Șoptitor (PV 10, 2×d6 ≥4, daune 2, armură 0, «spori»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Tâlhar cu pumnal (PV 9, 3×d6 ≥4, daune 1, armură 0, «înjunghiere»: ignoră armura, cost 2). Resursa se numește «libertate» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Storm Raven (duel: wind).
