# Sabatul de pe deal (duel: pădure) (`sabatul_de_pe_deal`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sabatul_de_pe_deal --seed 1`. Sursa implementării: `polytabletop-games/src/sabatul_de_pe_deal.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: duel de aventurieri în pădure, zaruri d100. Fiecare alege o clasă (alege k): Pândar, Cioară de seară, Tâlhar de drumuri. lovesc aruncă zarurile clasei (reușită la ≥ tn; daune = reușite × daune − armură, minim 1); parez = gardă (+2 armură până la tura ta, +1 farmec); special costă farmec și are efectul clasei; poțiune vindecă 4 PV. Reguli proprii: o lovitură de cel puțin 3 daune provoacă sângerare (1 PV pe tură, cel mult 3 ture); dacă toate zarurile (cel puțin două) arată 1, te rănești cu 1. Pierde cine ajunge la 0 PV; după 14 ture decide procentul de PV. Mutări: alege k, lovesc, lovesc+, parez, special, potiune, trec, tira.

Detalii: Clase: Pândar (PV 11, 2×d100 ≥46, daune 2, armură 0, «săgeată ascuțită»: ignoră armura, cost 2); Cioară de seară (PV 11, 1×d100 ≥51, daune 4, armură 1, «furt de suflu»: vindeci jumătate din daune, cost 2); Tâlhar de drumuri (PV 10, 1×d100 ≥51, daune 4, armură 1, «șoc»: dacă lovește, adversarul pierde tura, cost 2). Resursa se numește «farmec» (+1 pe tură, maximum 5). Poțiuni: 2.

Motor „duel de aventurieri” (t2_duel). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Sabbath on the Hill (duel: forest).
