# Yacht (`yacht`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play yacht --seed 1`. Sursa implementării: `polytabletop-games/src/yacht.rs`.

> Rezumat din `polytabletop list`: Yacht: 12 runde, până la 3 aruncări cu 5 zaruri, scrii scorul în 12 categorii (fiecare o dată); yacht = cinci egale = 50. Mutări: arunc, tine 3 3 5, scor casa.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Yacht (Asia / Marea Britanie), 2 jucători, din regulile publice ale jocului de zaruri.


* 12 runde pe jucător; în fiecare rundă: până la 3 aruncări cu 5 zaruri; după fiecare aruncare (primele

  două) poți **ține** unele zaruri (`tine 3 3 5`, `tine -` = niciunul) și rearunci restul, sau **scrii**

  scorul într-o categorie liberă (`scor casa`). După a treia aruncare trebuie să scrii (o categorie

  neîndeplinită primește 0).

* Aruncarea este o mutare separată, forțată (`arunc`), ca AI-ul să nu vadă zarurile viitoare.

* Categorii (fiecare o singură dată): unu…sase (suma zarurilor cu acea valoare), alegere (suma tuturor),

  patru-la-fel (suma celor patru zaruri egale), casa (trei + două, suma tuturor), scara-mica (1-2-3-4-5: 30),

  scara-mare (2-3-4-5-6: 30), yacht (cinci egale: 50). Nu există bonus pentru partea de sus.


AI: `evaluate` folosește doar scorurile, zarurile tale actuale și zarurile ținute (speranța exactă a

celei mai bune categorii după o aruncare a zarurilor rămase).
