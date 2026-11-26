# Armonia florilor (7×7) (`armonia`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Pai Sho”. Pornire: `polytabletop play armonia --seed 1`. Sursa implementării: `polytabletop-games/src/armonia.rs`.

> Reguli: Tablă 7×7; fiecare are trei feluri de piese a câte trei: trandafir (t, mută 1), lotus (l, până la 2), orhidee (o, până la 3). Trandafirul bate lotusul, lotusul orhideea, orhideea trandafirul; două la fel se anulează. O tură: pui o piesă din mână pe un punct gol din primele două rânduri ale tale sau lângă o piesă proprie, SAU muți ortogonal fără să sari. Două piese proprii diferite pe același rând/coloană fără nimic între ele formează o armonie. Câștigi cu un lanț de armonii de la coloana a la g (J1) sau de la rândul 1 la 7 (J2). Remiză după 160 de semi-mutări. Mutări: +t c3, c3-c5.
