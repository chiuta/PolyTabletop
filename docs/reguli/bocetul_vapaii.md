# Bocetul văpăii (duel: cenușă) (`bocetul_vapaii`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bocetul_vapaii --seed 1`. Sursa implementării: `polytabletop-games/src/bocetul_vapaii.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în cenușă, zaruri d6. Fiecare alege o clasă (alege k): Iscoadă, Luptător cu laț, Căpcăun flămând. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 dispreț); special costă dispreț și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: o lovitură de cel puțin 3 daune provoacă sângerare (1 PV pe tură, cel mult 3 ture); dacă toate zarurile (cel puțin două) arată 1, te rănești cu 1; sub jumătate din PV ai +1 daună. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Iscoadă (PV 9, 3×d6 ≥4, daune 1, armură 0, «săgeată ascuțită»: ignoră armura, cost 2); Luptător cu laț (PV 11, 2×d6 ≥4, daune 2, armură 1, «lovitură la tâmplă»: dacă lovește, adversarul pierde tura, cost 2); Căpcăun flămând (PV 10, 2×d6 ≥4, daune 2, armură 1, «sărutul umbrei»: vindeci jumătate din daune, cost 2). Resursa se numește «dispreț» (+1 pe tură, maximum 5). Poțiuni: 1.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Lament of the Blaze (duel: ash).
