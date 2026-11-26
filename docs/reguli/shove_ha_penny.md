# Shove ha'penny (`shove_ha_penny`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play shove_ha_penny --seed 1`. Sursa implementării: `polytabletop-games/src/shove_ha_penny.rs`.

> Rezumat din `polytabletop list`: Împingi 5 monede pe o tablă cu 9 paturi; un pat se închide după 3 monede; câștigi când ai închis toate cele 9 paturi. Mutări: p4, f7 (țintă + tip), apoi lansez.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Shove ha'penny (Marea Britanie), 2 jucători — simulare a jocului de îndemânare.


* Masa are 9 „paturi” (benzi) numerotate 1–9. Fiecare jucător împinge pe rând câte o monedă (5 pe rundă).

* Un jucător „umple” un pat când are în el 3 monede; o monedă aterizată într-un pat deja plin pentru

  împingător (3 ale lui) se creditează adversarului, dacă acesta are în pat mai puțin de 3; altfel e fără

  efect. Câștigă primul care are 3 monede în fiecare dintre cele 9 paturi.

* Fiindcă jocul real cere îndemânare, o împingere e **simulată**: alegi patul țintă și tipul loviturii;

  rezultatul se trage cu generatorul jocului. `precis` (`p4`): 50% exact, 15% un pat prea scurt, 10% un pat

  prea lung, 20% pe linie (nu contează), 5% cade de pe masă; `puternic` (`f4`): 40% exact, 5% scurt, 25%

  lung, 15% pe linie, 15% cade. O monedă care ar trece de patul 9 cade de pe masă.

* Împingerea se face în doi pași: alegi ținta, apoi mutarea forțată `lansez` o aruncă (AI-ul nu vede

  rezultatul înainte să aleagă). După 60 de runde fără câștigător câștigă cine a umplut mai mult

  (suma pătratelor cu plafon 3).


Mutări: `p4`, `f7` (țintă + tip), apoi `lansez`.
