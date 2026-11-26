# Duodecim scripta (`duodecim_scripta`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play duodecim_scripta --seed 1`. Sursa implementării: `polytabletop-games/src/dice_race.rs`.

> Rezumat din `polytabletop list`: Duodecim scripta (reconstituire modernă): 36 de puncte pe o singură cale, 15 piese pe jucător, 3 zaruri cu pași separați; un punct cu două piese e blocat; piesele singure lovite se întorc pe bară. Mutări: bar/3, 13/17, 36/off.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Motor comun pentru jocurile de tip „table” cu o singură cale: Long Nard (Narde) și Duodecim scripta.


* `LONG_NARD`: 24 de puncte, 15 piese pe jucător, două zaruri. Toate piesele pornesc din „cap”

  (punctul 1), cele două părți își parcurg cercul în același sens, începând din colțuri opuse. Nu se

  lovește: un punct ocupat de adversar e blocat. Dintr-un cap se ia o singură piesă pe tură (în prima

  tură a fiecăruia, cu dublă 3-3 sau 4-4, două). Nu poți forma șase puncte consecutive proprii dacă

  adversarul nu are nicio piesă mai înainte. Scoaterea începe când toate piesele sunt în ultimele șase

  puncte; primul care scoate 15 câștigă (fără „mars”).

* `DUODECIM`: reconstituire modernă (regulile originale nu s-au păstrat): 3 rânduri × 12 = 36 de puncte

  pe o singură cale parcursă de ambii în același sens, 15 piese fiecare, trei zaruri, fiecare zar un pas

  separat. Piesele intră de pe „bară” pe punctul egal cu zarul. Un punct cu două piese e blocat; o piesă

  singură lovită se întoarce pe bară. Scoaterea din ultimele șase puncte.

* Zarurile se aruncă automat în starea jocului; mutări: `7/10`, `bar/3`, `22/off`. Regula „joacă cât mai

  multe zaruri” se respectă.
