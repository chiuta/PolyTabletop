# Trei tipuri (stive 6×6) (`trei_tipuri`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „TZAAR”. Pornire: `polytabletop play trei_tipuri --seed 1`. Sursa implementării: `polytabletop-games/src/trei_tipuri.rs`.

> Reguli: Tablă 6×6, 18 piese pe jucător de trei tipuri (A 3, B 6, C 9). O piesă/stivă merge pe linie dreaptă pe prima celulă ocupată: capturează o stivă adversă de înălțime cel mult cât a ei sau se alipește unei stive proprii. O tură: o captură obligatorie, apoi opțional încă o acțiune (captură sau alipire) sau fin. Pierzi dacă nu mai ai un tip la vârf de stivă sau nu poți captura. Mutări: a1-a4, fin, predau.
