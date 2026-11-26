# Blocaje (curse cu blocuri) (`blocaje`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Malefiz”. Pornire: `polytabletop play blocaje --seed 1`. Sursa implementării: `polytabletop-games/src/b1_race.rs`.

> Reguli: Traseu circular de 40 de celule, 3 piese pe jucător. Piesa intră din curte cu orice zar. Pe celulele 7, 13, 27, 33 și 37 stau blocaje neutre, care nu pot fi trecute. Cine ajunge exact pe un blocaj îl ia și îl pune oriunde pe traseu (`bloc 12`, o celulă absolută liberă, nu sigură). Aterizarea pe o piesă adversară o trimite la start. Câștigi cu toate piesele în țintă (culoar de 4 celule); după 800 de semi-mutări, cine e mai avansat. Mutări: 1, a, bloc 12, pas.
