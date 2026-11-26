# Keno (`keno`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play keno --seed 1`. Sursa implementării: `polytabletop-games/src/keno.rs`.

> Rezumat din `polytabletop list`: Fiecare alege 5 numere din 80, apoi se extrag 20; plata (a noastră): 2 nimeriri = 1, 3 = 3, 4 = 20, 5 = 400 de puncte. 5 runde. Mutări: n17, random, extrag.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Keno (China / cazinouri), 2 jucători în întrecere, din regulile publice.


* Numerele 1–80. Într-o rundă fiecare jucător alege **5 numere** (J1, apoi J2); apoi se extrag 20 de numere

  din 80. Punctele rundei = tabelul de plată (propriul nostru, pentru 5 numere alese): 2 nimeriri = 1,

  3 = 3, 4 = 20, 5 = 400 de puncte; 0–1 nimeriri = 0.

* Se joacă 5 runde; câștigă cine are mai multe puncte (egal = remiză).

* Extragerea e o mutare forțată (`extrag`) făcută după ce ambii au ales, cu generatorul jocului, deci AI-ul

  nu poate ști numerele viitoare.


Mutări: `n17` (alegi numărul 17; numerele alese se dau în ordine crescătoare), `random` (completezi

aleatoriu până la 5), `extrag`.
