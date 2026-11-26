# Rithmomachia (`rithmomachia`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play rithmomachia --seed 1`. Sursa implementării: `polytabletop-games/src/rithmomachia.rs`.

> Rezumat din `polytabletop list`: Rithmomachia (varianta simplificată): 8×16, piese rotunde, triunghiuri și pătrate cu valori; capturi prin întâlnire, asalt și asediu; câștigi cu 8 capturi sau prinzând regele-piramidă. Mutări: a3-a5, a3xa5.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Rithmomachia (Europa medievală, „jocul filozofilor”), 2 jucători — varianta simplificată documentată.


* Tablă 8 coloane × 16 rânduri. Fiecare parte are 24 de piese: 8 rotunde, 8 triunghiuri, 8 pătrate

  (dintre care un pătrat e înlocuit de piramidă = regele). J1 = „pari” (X), J2 = „impari” (O).

  Numerele sunt cele tradiționale (pari: 2 4 6 8 4 16 36 64 / 6 20 42 72 9 25 49 81 / 15 45 91 153

  25 81 169 289; impari: 3 5 7 9 9 25 49 81 / 12 30 56 90 16 36 64 100 / 28 66 120 190 49 121 225 361),

  regele pari = 91, regele impari = 190. Așezarea pe rânduri (pătrate în spate, triunghiuri la mijloc,

  rotunde în față) este reconstituirea noastră; sursele variază.

* Mutarea: rotunda un pas pe diagonală; triunghiul exact două câmpuri ortogonal; pătratul exact trei

  câmpuri ortogonal; piramida (regele) poate muta ca oricare dintre cele trei. Fără salt (drumul liber).

* Capturi implementate: **întâlnire** (aterizezi pe o piesă adversă cu aceeași valoare), **asalt** (aterizezi

  pe o piesă adversă a cărei valoare = valoarea ta × lungimea mutării) și **asediu** (după mutarea ta, o piesă

  adversă ale cărei patru vecine ortogonale sunt piese ale tale sau marginea tablei e capturată).

  Ambuscada nu e implementată.

* Victorie: capturezi regele adversarului, sau 8 piese, sau adversarul nu mai are mutări. După 300 de

  mutări câștigă cine a capturat mai multă valoare (egal = remiză). Victoriile „proprii” cu progresii nu sunt

  implementate.


Mutări: `a3-a5`, cu captură `a3xa5`.
