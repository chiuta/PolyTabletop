# Iepurii și Elefantul (8×8) (`iepuri`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Arimaa”. Pornire: `polytabletop play iepuri --seed 1`. Sursa implementării: `polytabletop-games/src/iepuri.rs`.

> Reguli: Tablă 8×8, 16 piese pe jucător: elefant > cămilă > cai > câini > pisici > iepuri. Tura are până la 4 pași (fiecare e o mutare; fin încheie tura): pas simplu (iepurii nu dau înapoi), împingere sau tragere a unei piese adverse mai slabe (2 pași). O piesă lângă una mai tare și fără prieten e înghețată; pe capcanele c3 f3 c6 f6 fără prieten vecin dispare. Câștigi cu un iepure pe rândul advers. Mutări: e2n, d3e>n (împinge spre n), d3n<e3 (trage), fin.
