# De-a prinselea (Tag) (`de_a_prinselea`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play de_a_prinselea --seed 1`. Sursa implementării: `polytabletop-games/src/de_a_prinselea.rs`.

> Rezumat din `polytabletop list`: Urmărire pe un câmp 7×7: prinzătorul care ajunge adiacent schimbă rolurile (și stă o mutare); cel care nu e prinzător la sfârșitul rundei primește 1 punct; 15 runde. Mutări: n, s, e, v, ne, nv, se, sv, stai.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

De-a prinselea (Tag), 2 jucători — urmărire pe un câmp 7×7.


* J1 pornește ca «prinzător» (în colțul a1), J2 e fugarul (g7). Fiecare mutare: un pas ortogonal sau

  pe diagonală (`n`, `s`, `e`, `v`, `ne`, `nv`, `se`, `sv`) sau `stai`.

* Dacă prinzătorul termină mutarea adiacent (inclusiv diagonal) de celălalt, acesta e «prins»:

  rolurile se schimbă, iar noul prinzător stă pe loc o mutare («numărătoare»).

* După fiecare rundă (ambii au mutat) cel care nu e prinzător primește 1 punct. 15 runde; mai multe

  puncte câștigă, egal = remiză.


Mutări: `n`, `s`, `e`, `v`, `ne`, `nv`, `se`, `sv`, `stai`.
