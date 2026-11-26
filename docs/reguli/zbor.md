# Zbor (curse cu salturi) (`zbor`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Feixingqi (Flying chess)”. Pornire: `polytabletop play zbor --seed 1`. Sursa implementării: `polytabletop-games/src/b1_race.rs`.

> Reguli: Traseu circular de 40 de celule, 4 piese pe jucător. Piesa iese din curte la 5 sau 6; 6 dă încă o tură. Aterizarea pe o celulă de culoarea ta (celulă abs % 4 egală cu a ta) te trimite 4 celule înainte, iar de pe progresul 16 „zbori” 12 celule peste traseu (dacă celula e liberă). Aterizarea pe o piesă adversară o trimite în curte. Câștigi cu toate piesele în țintă (culoarul de 4 celule); după 800 de semi-mutări, cine e mai avansat. Mutări: 1, a (piesa de mutat), pas.
