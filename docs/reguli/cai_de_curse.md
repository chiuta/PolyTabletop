# Cai de curse (traseu 40) (`cai_de_curse`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Cờ cá ngựa”. Pornire: `polytabletop play cai_de_curse --seed 1`. Sursa implementării: `polytabletop-games/src/b1_race.rs`.

> Reguli: Traseu circular de 40 de celule, 4 piese pe jucător (J1: 1–4, J2: a–d). Zarul se aruncă automat; piesa iese din curte la 1 sau 6, iar 6 dă încă o tură. O piesă face 40 de pași, apoi intră pe culoarul propriu de 4 celule (cel mai din urmă = țintă). Aterizarea pe o piesă adversară o trimite în curte (intrările sunt celule sigure). Câștigi cu toate piesele în țintă; după 800 de semi-mutări, cine e mai avansat. Mutări: 1, a (piesa de mutat), pas.
