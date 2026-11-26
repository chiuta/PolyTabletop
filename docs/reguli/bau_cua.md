# Bầu cua (dovleac, crab, pește) (`bau_cua`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bau_cua --seed 1`. Sursa implementării: `polytabletop-games/src/bau_cua.rs`.

> Rezumat din `polytabletop list`: Bầu cua: trei zaruri cu cerb, dovleac, cocoș, pește, crab, creveți; parierul pune mize pe simboluri, bancherul aruncă; plată 1:1 per zar. 12 runde cu roluri alternante. Mutări: cerb 5, peste 10, gata, arunc.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Bầu cua cá cọp (Vietnam), 2 jucători, din regulile publice ale jocului de zaruri.


* Trei zaruri, fiecare cu șase simboluri: **cerb** (nai), **dovleac** (bầu), **cocoș** (gà), **pește** (cá),

  **crab** (cua), **creveți** (tôm). Un jucător este **bancherul**, celălalt **parierul**; rolurile alternează

  în fiecare rundă.

* Parierul pune mize (5 sau 10 jetoane) pe unul sau mai multe simboluri (maxim o miză pe simbol), apoi

  `gata`; bancherul aruncă zarurile (`arunc`, mutare forțată). Pentru fiecare zar cu simbolul parat, bancherul

  plătește miza (1:1) și parierul își păstrează miza; mizele pe simboluri care nu au ieșit se pierd către

  bancher.

* 12 runde, fiecare pornește cu 50 de jetoane; câștigă cine are mai multe la final (sau cine nu mai poate

  plăti). Miza totală dintr-o rundă nu poate depăși jetoanele parierului sau ale bancherului.


Mutări: `cerb 5`, `peste 10`, `gata`, `arunc`. Simboluri: cerb, dovleac, cocos, peste, crab, creveti.
