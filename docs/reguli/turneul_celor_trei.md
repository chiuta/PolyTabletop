# Turneul celor trei (duel: oraș) (`turneul_celor_trei`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play turneul_celor_trei --seed 1`. Sursa implementării: `polytabletop-games/src/turneul_celor_trei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în oraș, zaruri d6. Fiecare alege o clasă (alege k): Luptător cu laț, Vampir de hrubă, Hoț de acoperișuri. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 avânt); special costă avânt și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: dacă toate zarurile (cel puțin două) reușesc, daunele se dublează; garda întoarce 1 daună atacatorului. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Luptător cu laț (PV 11, 2×d6 ≥4, daune 2, armură 1, «lovitură la tâmplă»: dacă lovește, adversarul pierde tura, cost 2); Vampir de hrubă (PV 11, 2×d6 ≥4, daune 2, armură 1, «sărutul umbrei»: vindeci jumătate din daune, cost 2); Hoț de acoperișuri (PV 10, 3×d6 ≥4, daune 1, armură 0, «țintă fixă»: ignoră armura, cost 2). Resursa se numește «avânt» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Tournament of Three (duel: city).
