# Batak (Turcia) (`batak`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play batak --seed 1`. Sursa implementării: `polytabletop-games/src/batak.rs`.

> Rezumat din `polytabletop list`: Batak adaptat la 2 jucători (13 cărți fiecare): licitezi levatele, alegi atu (pică implicit), cine licitează trebuie să facă cel puțin cât a promis. Mutări: 6, pas, atu h, Ah.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Batak (Turcia), 2 jucători — adaptare a jocului de levate cu licitație (în original, 4 jucători).


* Pachet de 52, câte 13 cărți fiecăruia; ceilalte 26 rămân nefolosite și nevăzute (ca la celelalte jocuri

  de levate pe doi ale colecției).

* **Licitația**: cel care nu e dealer spune un număr de levate între 5 și 13 sau `pas`; dealerul poate licita

  mai mult sau da pas (dacă primul a dat pas, dealerul trebuie să liciteze). Cine licitează cel mai mult

  alege **atuul** (oricare culoare) și conduce.

* **Jocul**: se răspunde obligatoriu la culoare și, dacă poți, cu o carte mai mare decât cea care ia

  momentan levata; fără culoare trebuie să tai cu atu (și să suprataie dacă poți); fără atu, orice. Ia cea mai

  mare carte din culoarea condusă sau cel mai mare atu; câștigătorul conduce.

* **Scor**: licitatorul primește +contract dacă ia cel puțin atâtea levate, altfel −contract; celălalt

  primește câte un punct pe levată luată. Câștigă primul care ajunge la 41 de puncte (după cel mult 30 de

  mâini câștigă cine are mai multe).


Mutări: `6` (licitație), `pas`, `atu h` (culoarea atuului: h d s c), carte (`Ah`, `7c`), `urmatoarea`.
