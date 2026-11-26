# Marmure (curse cu scurtătură) (`marmure`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Aggravation”. Pornire: `polytabletop play marmure --seed 1`. Sursa implementării: `polytabletop-games/src/b1_race.rs`.

> Reguli: Traseu circular de 40 de celule, 4 bile pe jucător. Bila iese din curte la 1 sau 6; 6 dă încă o tură. Dacă o bilă ajunge exact pe celula de progres 12, poți alege scurtătura de la 12 la 30 (`1!`, dacă celula 30 e liberă). Aterizarea pe o bilă adversă o trimite în curte. Câștigi cu toate bilele în țintă (culoar de 4 celule); după 800 de semi-mutări, cine e mai avansat. Mutări: 1, 1! (scurtătura), a, pas.
