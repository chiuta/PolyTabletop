# Jocul gâștei (Game of the Goose) (`goose`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play goose --seed 1`. Sursa implementării: `polytabletop-games/src/goose.rs`.

> Rezumat din `polytabletop list`: Cărare de 63 de căsuțe, două zaruri: gâște (mai mergi o dată), pod, han, fântână, labirint, închisoare, moarte; ajungi exact pe 63. Fântâna și închisoarea costă 3 ture. Mutări: arunc, astept.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Jocul gâștei (Game of the Goose, Italia), 2 jucători, din regulile publice.


* Cărare de 63 de căsuțe, două zaruri; pornești din 0 și ajungi **exact** pe 63 (dacă treci peste, te

  întorci cu surplusul).

* Căsuțe speciale: **gâscă** (5, 9, 14, 18, 23, 27, 32, 36, 41, 45, 50, 54, 59): mai mergi o dată cu

  aceeași sumă; **pod** (6): sari pe 12; **han** (19): pierzi o tură; **fântână** (31): pierzi trei ture;

  **labirint** (42): te întorci pe 30; **închisoare** (52): pierzi trei ture; **moartea** (58): te

  întorci pe 1. Prima aruncare 3+6 duce pe 26, 4+5 pe 53.

* Variante alese de noi: în loc de „stai până vine altul” (care poate bloca doi jucători), fântâna și

  închisoarea costă trei ture; nu se schimbă locurile când ajungi pe căsuța adversarului.


Mutări: `arunc` (zarurile se aruncă în starea jocului), `astept` (tură pierdută).
