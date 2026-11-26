# Pong hau k'i (`pong_hau_kiki`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pong_hau_kiki --seed 1`. Sursa implementării: `polytabletop-games/src/pong_hau_kiki.rs`.

> Rezumat din `polytabletop list`: Pong hau k'i (China): 5 puncte, 2 piese pe jucător; muți pe un punct vecin liber; câștigi blocând adversarul. Mutări: 1-3.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Pong hau k'i (China), 2 jucători, din regulile publice.


* Tabla are cinci puncte: două sus (1, 2), centrul (3) și două jos (4, 5); liniile unesc 1–2, 4–5

  și cele două diagonale prin centru (1–3–5 și 2–3–4).

* Fiecare jucător are două piese: J1 (X) pe 1 și 2, J2 (O) pe 4 și 5; centrul e gol. J1 începe.

* Muți o piesă pe un punct vecin liber (pe o linie). Pierde cine nu mai poate muta (e blocat).

* Remiză la a treia repetare a aceleiași poziții sau după 100 de mutări.


Mutări: `1-3` (de pe punctul 1 pe punctul 3).
