# Ascunselea (Hide-and-seek) (`ascunselea`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ascunselea --seed 1`. Sursa implementării: `polytabletop-games/src/ascunselea.rs`.

> Rezumat din `polytabletop list`: Ascunselea pe o curte 3×3, 4 runde: ascunzătorul alege în secret o ascunzătoare, căutătorul are 3 încercări cu indiciu cald/rece; mai repede = mai multe puncte. Mutări: ascunde b2, cauta b2.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Ascunselea (Hide-and-seek), 2 jucători — adaptare pe un curte 3×3.


* 4 runde; în rundele 1 și 3 ascunde J1, în 2 și 4 ascunde J2. Ascunzătorul alege în secret una

  din cele 9 ascunzători (a1–c3) cu `ascunde b2`.

* Căutătorul are 3 încercări (`cauta b2`). După o greșeală primește un indiciu public: «cald» dacă

  ascunzătoarea e vecină (inclusiv diagonal) cu locul ales, «rece» altfel.

* Găsit din prima încercare: 3 puncte pentru căutător, din a doua 2, din a treia 1; dacă nu e găsit,

  ascunzătorul primește 2 puncte. Cine are mai multe puncte după 4 runde câștigă.

* Informație ascunsă: locul ales; `render` îl arată doar ascunzătorului (sau la final de rundă).

  AI-ul nu citește locul ascunzătorului atunci când caută.
