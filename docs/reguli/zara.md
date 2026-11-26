# Zara (joc) (`zara`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zara --seed 1`. Sursa implementării: `polytabletop-games/src/zara.rs`.

> Rezumat din `polytabletop list`: Zara (reconstituire): fiecare strigă un număr 4–17, se aruncă 3 zaruri; cine a strigat suma ia atâtea puncte; 10 runde. Mutări: n13, arunc.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Zara (Italia medievală), 2 jucători — reconstituire modernă a jocului de zaruri cu numere strigate.


Sursele păstrează doar ideea: un jucător strigă un număr, aruncă trei zaruri, câștigă dacă suma lor e

numărul strigat. Aici, în 10 runde:

* J1 strigă un număr între 4 și 17 (`n13`), apoi J2 strigă **alt** număr între 4 și 17; apoi se aruncă trei

  zaruri (`arunc`, mutare forțată).

* Cine a strigat suma aruncată primește **atâtea puncte cât numărul strigat** (3 și 18 nu pot fi strigate,

  fiind „azar”); dacă nimeni n-a nimerit, runda nu dă puncte.

* După 10 runde câștigă cine are mai multe puncte (egal = remiză). Rundele alternează cine strigă primul.


Mutări: `n13`, `arunc`.
