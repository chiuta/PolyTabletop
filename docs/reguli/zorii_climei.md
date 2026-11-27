# Zorii Climei (`zorii_climei`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play zorii_climei --seed 1`. Sursa implementării: `polytabletop-games/src/c2_surv.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de supraviețuire: 4 oameni, resurse energie, fonduri, voință; echipa (J1: Delegata și Inventatorul, câte 2 acțiuni pe rând) adună resurse (cauta i: 0=Rețeaua (+2 energie); 1=Banca (+3 fonduri); 2=Piața (+2 voință); 3=Laboratorul (+2 fonduri); fiecare loc o dată pe rundă), lucrează la proiectul „Tranziția” (lucru: 2 fonduri → +1; trebuie 9) și întreține moralul (ingrij: 1 energie → +1 moral, maxim 6). J2 este Jocul: eveniment (forțat) — hrănire (energie: jumătate din oameni, rotunjit în sus), apoi criza anunțată public (un anumit număr dintr-o resursă, altfel moral pierdut). Câștigă echipa (J1) terminând proiectul în 11 runde; Jocul (J2) câștigă dacă moralul sau oamenii se termină. Mutări: cauta i, lucru, ingrij, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
