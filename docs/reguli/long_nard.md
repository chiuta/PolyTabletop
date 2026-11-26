# Long Nard (Narde) (`long_nard`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play long_nard --seed 1`. Sursa implementării: `polytabletop-games/src/dice_race.rs`.

> Rezumat din `polytabletop list`: Narde lungi: 15 piese pe jucător, 24 de puncte pe un singur cerc, fără lovire (punct blocat de adversar), o singură piesă din cap pe tură; câștigă cine scoate primul cele 15 piese. Mutări: 13/17, 24/off.

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
