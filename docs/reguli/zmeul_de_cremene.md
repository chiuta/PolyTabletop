# Zmeul de cremene (duel: peșteră) (`zmeul_de_cremene`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zmeul_de_cremene --seed 1`. Sursa implementării: `polytabletop-games/src/zmeul_de_cremene.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în peșteră, zaruri d6. Fiecare alege o clasă (alege k): Ghicitor, Luptător cu laț, Cleric bătrân. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 foc); special costă foc și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: sub jumătate din PV ai +1 daună; dacă toate zarurile (cel puțin două) arată 1, te rănești cu 1. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Ghicitor (PV 9, 2×d6 ≥4, daune 2, armură 0, «otravă lentă»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Luptător cu laț (PV 10, 2×d6 ≥4, daune 2, armură 1, «lovitură la tâmplă»: dacă lovește, adversarul pierde tura, cost 2); Cleric bătrân (PV 13, 2×d6 ≥4, daune 1, armură 2, «rugă»: vindecă 4 PV, cost 2). Resursa se numește «foc» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Flint Drake (duel: cave).
