# Umbre cu lame (duel: noapte) (`umbre_cu_lame`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play umbre_cu_lame --seed 1`. Sursa implementării: `polytabletop-games/src/umbre_cu_lame.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în noapte, zaruri d6. Fiecare alege o clasă (alege k): Gladiator, Mag de furtună, Bastonar. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 tăcere); special costă tăcere și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: dacă toate zarurile (cel puțin două) reușesc, daunele se dublează; garda întoarce 1 daună atacatorului. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Gladiator (PV 14, 2×d6 ≥4, daune 2, armură 1, «ofensivă»: +2 daune pe reușită, cost 2); Mag de furtună (PV 9, 2×d6 ≥4, daune 2, armură 0, «otravă lentă»: otravă 2: 1 PV pe tură, 2 ture, cost 2); Bastonar (PV 10, 2×d6 ≥4, daune 2, armură 1, «bubuitură de clopot»: dacă lovește, adversarul pierde tura, cost 2). Resursa se numește «tăcere» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Shadows with Blades (duel: night).
