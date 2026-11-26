# Pardon (curse cu cărți) (`scuze`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Sorry!”. Pornire: `polytabletop play scuze --seed 1`. Sursa implementării: `polytabletop-games/src/b1_race.rs`.

> Reguli: Traseu circular de 40 de celule, 3 piese pe jucător; o piesă iese din curte cu cartea 1 sau 2. Se trage o carte automat (1, 2, 3, −4, 5, 7, 8, 10, 11, 12 sau „pardon”). Piesa merge numărul de pași (−4 înapoi). La 11 poți schimba locul unei piese proprii de pe traseu cu o piesă adversă (`1xa`); la „pardon” o piesă din curte ia locul unei piese adverse, trimițând-o în curte (`1sa`). Aterizarea pe o piesă adversă o trimite în curte. Câștigi cu toate piesele în țintă; după 800 de semi-mutări, cine e mai avansat. Mutări: 1, 1xa, 1sa, pas.
