# Piquet (Franța) (`piquet`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play piquet --seed 1`. Sursa implementării: `polytabletop-games/src/piquet.rs`.

> Rezumat din `polytabletop list`: Piquet (32 de cărți): schimb de cărți, declarații (punct, secvențe, seturi) și 12 levate; variantă proprie de punctare a jocului. Mutări: d7h, gata, Ah, urmatoarea.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Piquet (Franța), 2 jucători, din regulile publice ale partidei clasice (șase maini).


* 32 de cărți (7–A). Fiecare primește 12; restul de 8 formează talonul. Cel care nu e dealer = „elder”

  (mâna întâi). O partie = 6 maini, dealerul alternează; câștigă cine are mai multe puncte.

* **Schimbul**: elderul aruncă 1–5 cărți (`d7h`, câte una, apoi `gata`) și trage tot atâtea din talon; apoi

  dealerul aruncă 0–(cărțile rămase în talon, max. 5) și trage.

* **Declarațiile** (calculate automat din mâinile definitive; câștigătorul fiecărei categorii punctează):

  *punct* (cele mai multe cărți într-o culoare, la egalitate suma valorilor A 11, figuri 10, restul pip;

  punctează numărul de cărți), *secvență* (serii de cel puțin 3 cărți consecutive într-o culoare: 3→3, 4→4,

  5→15, 6→16, 7→17, 8→18; cea mai lungă / mai mare decide, câștigătorul punctează toate secvențele lui), *set*

  (trei sau patru cărți de 10, J, Q, K sau A: trio 3, quatorze 14; câștigătorul punctează toate seturile lui).

  La egal fără câștigător, nimeni nu punctează. Variantă implementată: fără carte blanche și fără carte rouge.

* **Jocul**: elderul conduce, se răspunde obligatoriu la culoare, ia cea mai mare carte din culoarea condusă.

  Puncte în joc (variantă documentată, sursele diferă): cine conduce cu o carte de 10 sau mai mare ia 1 punct;

  cine ia levata răspunzând ia 1 punct; ultima levată +1. Cărțile: cine ia mai multe levate +10; capot (toate cele

  12) +40 în loc de 10. **Repique**: 30 de puncte din declarații înainte ca adversarul să aibă vreunul → +60;

  **pique**: elderul ajunge la 30 (declarații + joc) înainte ca dealerul să aibă vreun punct → +30.


Mutări: `d7h` (aruncă 7♥), `gata`, carte (`Ah`, `10c`), `urmatoarea`. Informație ascunsă: vezi `p3_comun`.
