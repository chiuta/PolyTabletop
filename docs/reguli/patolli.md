# Patolli (`patolli`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play patolli --seed 1`. Sursa implementării: `polytabletop-games/src/patolli.rs`.

> Rezumat din `polytabletop list`: Patolli (reconstituire): cale în cruce de 52 de pătrate, 6 piese pe jucător, 5 boabe (numărul fețelor marcate = pași); scoți piesele exact la sfârșit. Mutări: piesa 0, piesa 17.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Patolli (Mexic precolumbian), 2 jucători — reconstituire modernă din regulile publice.


Regulile originale nu s-au păstrat integral; implementăm varianta reconstituită cel mai des:

* Cărare circulară de 52 de pătrate (în forma tradițională de cruce); fiecare jucător are 6 piese.

* Cinci boabe, fiecare cu o față marcată; numărul de fețe marcate = pași (0 = nicio mutare, 5 = 10 pași).

* Piesele intră pe cale la pătratul de start al jucătorului (J1 pe 1, J2 pe 27), parcurg exact 52 de

  pătrate și ies când ating exact pătratul 52 al drumului lor (nu se poate depăși).

* Pe un pătrat ocupat de o piesă proprie nu se poate ateriza; aterizând pe o piesă adversă o trimiți

  înapoi (afară), în afara pătratelor sigure (cele patru colțuri ale crucii: 1, 14, 27, 40), unde

  piesele coexistă.

* Câștigă primul care scoate toate cele 6 piese; după 800 de mutări câștigă cine e mai avansat.


Mutări: `piesa 0` (piesa de pe poziția 0 = intră pe cale), `piesa 17` (piesa aflată la pasul 17 din drumul

tău), `pas` (când zarul dă 0 sau nu ai mutări). Zarul se vede în starea jocului.
