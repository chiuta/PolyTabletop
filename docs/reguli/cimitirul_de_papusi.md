# Cimitirul de păpuși (duel: morminte) (`cimitirul_de_papusi`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cimitirul_de_papusi --seed 1`. Sursa implementării: `polytabletop-games/src/cimitirul_de_papusi.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în morminte, zaruri d10. Fiecare alege o clasă (alege k): Rob liber, Vânător, Duh însetat. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 șoaptă); special costă șoaptă și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: o lovitură de cel puțin 3 daune provoacă sângerare (1 PV pe tură, cel mult 3 ture); sub jumătate din PV ai +1 daună. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Rob liber (PV 11, 2×d10 ≥6, daune 2, armură 1, «bubuitură de clopot»: dacă lovește, adversarul pierde tura, cost 2); Vânător (PV 10, 3×d10 ≥6, daune 1, armură 0, «țintă fixă»: ignoră armura, cost 2); Duh însetat (PV 11, 2×d10 ≥6, daune 2, armură 1, «sărutul umbrei»: vindeci jumătate din daune, cost 2). Resursa se numește «șoaptă» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Doll Graveyard (duel: tombs).
